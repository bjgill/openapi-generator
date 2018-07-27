package org.openapitools.codegen;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.media.ObjectSchema;
import io.swagger.v3.oas.models.media.*;
// import io.swagger.v3.oas.models.parameters.BodyParameter;
import io.swagger.v3.oas.models.parameters.Parameter;
// import io.swagger.v3.oas.models.properties.*;
import io.swagger.v3.oas.models.Paths;

import io.swagger.v3.core.util.Json;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class InlineModelResolver {
    private OpenAPI openapi;
    private boolean skipMatches;
    static Logger LOGGER = LoggerFactory.getLogger(InlineModelResolver.class);

    Map<String, Schema> addedModels = new HashMap<String, Schema>();
    Map<String, String> generatedSignature = new HashMap<String, String>();

    public void flatten(OpenAPI openapi) {
        this.openapi = openapi;

        if (openapi.getDefinitions() == null) {
            openapi.setDefinitions(new HashMap<String, Schema>());
        }

        // operations
        Map<String, Path> paths = openapi.getPaths();
        Map<String, Schema> models = openapi.getDefinitions();

        if (paths != null) {
            for (String pathname : paths.keySet()) {
                Path path = paths.get(pathname);

                for (Operation operation : path.getOperations()) {
                    List<Parameter> parameters = operation.getParameters();

                    if (parameters != null) {
                        for (Parameter parameter : parameters) {
                            // if (parameter instanceof BodyParameter) {
                            //     BodyParameter bp = (BodyParameter) parameter;
                            //     if (bp.getSchema() != null) {
                            //         Schema model = bp.getSchema();
                            //         if (model instanceof Schema) {
                            //             Schema obj = (Schema) model;
                            //             if (obj.getType() == null || "object".equals(obj.getType())) {
                            //                 if (obj.getProperties() != null && obj.getProperties().size() > 0) {
                            //                     flattenProperties(obj.getProperties(), pathname);
                            //                     String modelName = resolveModelName(obj.getTitle(), bp.getName());
                            //                     bp.setSchema(new RefModel(modelName));
                            //                     addGenerated(modelName, model);
                            //                     swagger.addDefinition(modelName, model);
                            //                 }
                            //             }
                            //         } else if (model instanceof ArrayModel) {
                            //             ArrayModel am = (ArrayModel) model;
                            //             Schema inner = am.getItems();

                            //             if (inner instanceof ObjectSchema) {
                            //                 ObjectSchema op = (ObjectSchema) inner;
                            //                 if (op.getProperties() != null && op.getProperties().size() > 0) {
                            //                     flattenProperties(op.getProperties(), pathname);
                            //                     String modelName = resolveModelName(op.getTitle(), bp.getName());
                            //                     Schema innerModel = modelFromProperty(op, modelName);
                            //                     String existing = matchGenerated(innerModel);
                            //                     if (existing != null) {
                            //                         RefProperty refProperty = new RefProperty(existing);
                            //                         refProperty.setRequired(op.getRequired());
                            //                         am.setItems(refProperty);
                            //                     } else {
                            //                         RefProperty refProperty = new RefProperty(modelName);
                            //                         refProperty.setRequired(op.getRequired());
                            //                         am.setItems(refProperty);
                            //                         addGenerated(modelName, innerModel);
                            //                         swagger.addDefinition(modelName, innerModel);
                            //                     }
                            //                 }
                            //             }
                            //         }
                            //     }
                            // }
                        }
                    }
                    Map<String, Response> responses = operation.getResponses();
                    if (responses != null) {
                        for (String key : responses.keySet()) {
                            Response response = responses.get(key);
                            if (response.getSchema() != null) {
                                Schema property = response.getSchema();
                                if (property instanceof ObjectSchema) {
                                    ObjectSchema op = (ObjectSchema) property;
                                    if (op.getProperties() != null && op.getProperties().size() > 0) {
                                        String modelName = resolveModelName(op.getTitle(), "inline_response_" + key);
                                        Schema model = modelFromProperty(op, modelName);
                                        String existing = matchGenerated(model);
                                        if (existing != null) {
                                            Schema refProperty = this.makeRefProperty(existing, property);
                                            refProperty.setRequired(op.getRequired());
                                            response.setSchema(refProperty);
                                        } else {
                                            Schema refProperty = this.makeRefProperty(modelName, property);
                                            refProperty.setRequired(op.getRequired());
                                            response.setSchema(refProperty);
                                            addGenerated(modelName, model);
                                            openapi.addDefinition(modelName, model);
                                        }
                                    }
                                } else if (property instanceof ArraySchema) {
                                    ArraySchema ap = (ArraySchema) property;
                                    Schema inner = ap.getItems();

                                    if (inner instanceof ObjectSchema) {
                                        ObjectSchema op = (ObjectSchema) inner;
                                        if (op.getProperties() != null && op.getProperties().size() > 0) {
                                            flattenProperties(op.getProperties(), pathname);
                                            String modelName = resolveModelName(op.getTitle(),
                                                    "inline_response_" + key);
                                            Schema innerModel = modelFromProperty(op, modelName);
                                            String existing = matchGenerated(innerModel);
                                            if (existing != null) {
                                                Schema refProperty = this.makeRefProperty(existing, op);
                                                refProperty.setRequired(op.getRequired());
                                                ap.setItems(refProperty);
                                            } else {
                                                Schema refProperty = this.makeRefProperty(modelName, op);
                                                refProperty.setRequired(op.getRequired());
                                                ap.setItems(refProperty);
                                                addGenerated(modelName, innerModel);
                                                openapi.addDefinition(modelName, innerModel);
                                            }
                                        }
                                    }
                                } else if (property instanceof MapSchema) {
                                    MapSchema mp = (MapSchema) property;

                                    Schema innerProperty = mp.getAdditionalProperties();
                                    if (innerProperty instanceof ObjectSchema) {
                                        ObjectSchema op = (ObjectSchema) innerProperty;
                                        if (op.getProperties() != null && op.getProperties().size() > 0) {
                                            flattenProperties(op.getProperties(), pathname);
                                            String modelName = resolveModelName(op.getTitle(),
                                                    "inline_response_" + key);
                                            Schema innerModel = modelFromProperty(op, modelName);
                                            String existing = matchGenerated(innerModel);
                                            if (existing != null) {
                                                RefProperty refProperty = new RefProperty(existing);
                                                refProperty.setRequired(op.getRequired());
                                                mp.setAdditionalProperties(refProperty);
                                            } else {
                                                RefProperty refProperty = new RefProperty(modelName);
                                                refProperty.setRequired(op.getRequired());
                                                mp.setAdditionalProperties(refProperty);
                                                addGenerated(modelName, innerModel);
                                                openapi.addDefinition(modelName, innerModel);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // definitions
        if (models != null) {
            List<String> modelNames = new ArrayList<String>(models.keySet());
            for (String modelName : modelNames) {
                Schema model = models.get(modelName);
                if (model instanceof Schema) {
                    Schema m = (Schema) model;

                    Map<String, Schema> properties = m.getProperties();
                    flattenProperties(properties, modelName);
                    fixStringModel(m);
                } else if (model instanceof ArrayModel) {
                    ArrayModel m = (ArrayModel) model;
                    Schema inner = m.getItems();
                    if (inner instanceof ObjectSchema) {
                        ObjectSchema op = (ObjectSchema) inner;
                        if (op.getProperties() != null && op.getProperties().size() > 0) {
                            String innerModelName = resolveModelName(op.getTitle(), modelName + "_inner");
                            Schema innerModel = modelFromProperty(op, innerModelName);
                            String existing = matchGenerated(innerModel);
                            if (existing == null) {
                                openapi.addDefinition(innerModelName, innerModel);
                                addGenerated(innerModelName, innerModel);
                                RefProperty refProperty = new RefProperty(innerModelName);
                                refProperty.setRequired(op.getRequired());
                                m.setItems(refProperty);
                            } else {
                                RefProperty refProperty = new RefProperty(existing);
                                refProperty.setRequired(op.getRequired());
                                m.setItems(refProperty);
                            }
                        }
                    }
                } else if (model instanceof ComposedModel) {
                    ComposedModel m = (ComposedModel) model;
                    if (m.getChild() != null) {
                        Map<String, Schema> properties = m.getChild().getProperties();
                        flattenProperties(properties, modelName);
                    }
                }
            }
        }
    }

    /**
     * This function fix models that are string (mostly enum). Before this fix, the example
     * would look something like that in the doc: "\"example from def\""
     * @param m Schema implementation
     */
    private void fixStringModel(Schema m) {
        if (m.getType() != null && m.getType().equals("string") && m.getExample() != null) {
            String example = m.getExample().toString();
            if (example.substring(0, 1).equals("\"") &&
                    example.substring(example.length() - 1).equals("\"")) {
                m.setExample(example.substring(1, example.length() - 1));
            }
        }
    }

    private String resolveModelName(String title, String key) {
        if (title == null) {
            return uniqueName(key);
        } else {
            return uniqueName(title);
        }
    }

    public String matchGenerated(Schema model) {
        if (this.skipMatches) {
            return null;
        }
        String json = Json.pretty(model);
        if (generatedSignature.containsKey(json)) {
            return generatedSignature.get(json);
        }
        return null;
    }

    public void addGenerated(String name, Schema model) {
        generatedSignature.put(Json.pretty(model), name);
    }

    public String uniqueName(String key) {
        int count = 0;
        boolean done = false;
        key = key.replaceAll("[^a-z_\\.A-Z0-9 ]", ""); // FIXME: a parameter
                                                       // should not be
                                                       // assigned. Also declare
                                                       // the methods parameters
                                                       // as 'final'.
        while (!done) {
            String name = key;
            if (count > 0) {
                name = key + "_" + count;
            }
            if (openapi.getDefinitions() == null) {
                return name;
            } else if (!openapi.getDefinitions().containsKey(name)) {
                return name;
            }
            count += 1;
        }
        return key;
    }

    public void flattenProperties(Map<String, Schema> properties, String path) {
        if (properties == null) {
            return;
        }
        Map<String, Schema> propsToUpdate = new HashMap<String, Schema>();
        Map<String, Schema> modelsToAdd = new HashMap<String, Schema>();
        for (String key : properties.keySet()) {
            Schema property = properties.get(key);
            if (property instanceof ObjectSchema && ((ObjectSchema) property).getProperties() != null
                    && ((ObjectSchema) property).getProperties().size() > 0) {

                ObjectSchema op = (ObjectSchema) property;

                String modelName = resolveModelName(op.getTitle(), path + "_" + key);
                Schema model = modelFromProperty(op, modelName);

                String existing = matchGenerated(model);

                if (existing != null) {
                    RefProperty refProperty = new RefProperty(existing);
                    refProperty.setRequired(op.getRequired());
                    propsToUpdate.put(key, refProperty);
                } else {
                    RefProperty refProperty = new RefProperty(modelName);
                    refProperty.setRequired(op.getRequired());
                    propsToUpdate.put(key, refProperty);
                    modelsToAdd.put(modelName, model);
                    addGenerated(modelName, model);
                    openapi.addDefinition(modelName, model);
                }
            } else if (property instanceof ArraySchema) {
                ArraySchema ap = (ArraySchema) property;
                Schema inner = ap.getItems();

                if (inner instanceof ObjectSchema) {
                    ObjectSchema op = (ObjectSchema) inner;
                    if (op.getProperties() != null && op.getProperties().size() > 0) {
                        flattenProperties(op.getProperties(), path);
                        String modelName = resolveModelName(op.getTitle(), path + "_" + key);
                        Schema innerModel = modelFromProperty(op, modelName);
                        String existing = matchGenerated(innerModel);
                        if (existing != null) {
                            RefProperty refProperty = new RefProperty(existing);
                            refProperty.setRequired(op.getRequired());
                            ap.setItems(refProperty);
                        } else {
                            RefProperty refProperty = new RefProperty(modelName);
                            refProperty.setRequired(op.getRequired());
                            ap.setItems(refProperty);
                            addGenerated(modelName, innerModel);
                            openapi.addDefinition(modelName, innerModel);
                        }
                    }
                }
            } else if (property instanceof MapSchema) {
                MapSchema mp = (MapSchema) property;
                Schema inner = mp.getAdditionalProperties();

                if (inner instanceof ObjectSchema) {
                    ObjectSchema op = (ObjectSchema) inner;
                    if (op.getProperties() != null && op.getProperties().size() > 0) {
                        flattenProperties(op.getProperties(), path);
                        String modelName = resolveModelName(op.getTitle(), path + "_" + key);
                        Schema innerModel = modelFromProperty(op, modelName);
                        String existing = matchGenerated(innerModel);
                        if (existing != null) {
                            RefProperty refProperty = new RefProperty(existing);
                            refProperty.setRequired(op.getRequired());
                            mp.setAdditionalProperties(refProperty);
                        } else {
                            RefProperty refProperty = new RefProperty(modelName);
                            refProperty.setRequired(op.getRequired());
                            mp.setAdditionalProperties(refProperty);
                            addGenerated(modelName, innerModel);
                            openapi.addDefinition(modelName, innerModel);
                        }
                    }
                }
            }
        }
        if (propsToUpdate.size() > 0) {
            for (String key : propsToUpdate.keySet()) {
                properties.put(key, propsToUpdate.get(key));
            }
        }
        for (String key : modelsToAdd.keySet()) {
            openapi.addDefinition(key, modelsToAdd.get(key));
            this.addedModels.put(key, modelsToAdd.get(key));
        }
    }

    @SuppressWarnings("static-method")
    public Schema modelFromProperty(ArraySchema object, @SuppressWarnings("unused") String path) {
        String description = object.getDescription();
        String example = null;

        Object obj = object.getExample();
        if (obj != null) {
            example = obj.toString();
        }

        Schema inner = object.getItems();
        if (inner instanceof ObjectSchema) {
            ArrayModel model = new ArrayModel();
            model.setDescription(description);
            model.setExample(example);
            model.setItems(object.getItems());
            return model;
        }

        return null;
    }

    public Schema modelFromProperty(ObjectSchema object, String path) {
        String description = object.getDescription();
        String example = null;

        Object obj = object.getExample();
        if (obj != null) {
            example = obj.toString();
        }
        String name = object.getName();
        Xml xml = object.getXml();
        Map<String, Schema> properties = object.getProperties();

        Schema model = new Schema();
        model.setDescription(description);
        model.setExample(example);
        model.setName(name);
        model.setXml(xml);

        if (properties != null) {
            flattenProperties(properties, path);
            model.setProperties(properties);
        }

        return model;
    }

    @SuppressWarnings("static-method")
    public Schema modelFromProperty(MapSchema object, @SuppressWarnings("unused") String path) {
        String description = object.getDescription();
        String example = null;

        Object obj = object.getExample();
        if (obj != null) {
            example = obj.toString();
        }

        ArrayModel model = new ArrayModel();
        model.setDescription(description);
        model.setExample(example);
        model.setItems(object.getAdditionalProperties());

        return model;
    }

    /**
     * Make a RefProperty
     *
     * @param ref new property name
     * @param property Schema
     * @return {@link Schema} A constructed OpenAPI property
     */
    public Schema makeRefProperty(String ref, Schema property) {
        RefProperty newProperty = new RefProperty(ref);
        this.copyVendorExtensions(property, newProperty);
        return newProperty;
    }

    /**
     * Copy vendor extensions from Schema to another Schema
     *
     * @param source source property
     * @param target target property
     */
    public void copyVendorExtensions(Schema source, Schema target) {
        Map<String, Object> vendorExtensions = source.getVendorExtensions();
        for (String extName : vendorExtensions.keySet()) {
            target.setVendorExtension(extName, vendorExtensions.get(extName));
        }
    }

    public boolean isSkipMatches() {
        return skipMatches;
    }

    public void setSkipMatches(boolean skipMatches) {
        this.skipMatches = skipMatches;
    }

}
