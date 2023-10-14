// DEPRECATED/BACKCOMPATIBILITY START
void setJointMaxForce(int64_t objectHandle, double forceOrTorque);
double getJointMaxForce(int64_t jointHandle);
int64_t createPureShape(int64_t primitiveType, int64_t options,
                        std::vector<double> sizes, double mass,
                        std::optional<std::vector<int64_t>> precision = {});
void removeObject(int64_t objectHandle);
std::tuple<std::vector<uint8_t>, std::vector<int64_t>>
getVisionSensorDepthBuffer(int64_t sensorHandle,
                           std::optional<std::vector<int64_t>> pos = {},
                           std::optional<std::vector<int64_t>> size = {});
std::tuple<std::vector<uint8_t>, std::vector<int64_t>>
getVisionSensorCharImage(int64_t sensorHandle,
                         std::optional<std::vector<int64_t>> pos = {},
                         std::optional<std::vector<int64_t>> size = {});
void setVisionSensorCharImage(int64_t sensorHandle, std::vector<uint8_t> image);

std::vector<int64_t> getObjectSelection();
void setObjectSelection(std::vector<double> objectHandles);
// DEPRECATED/BACKCOMPATIBILITY START

// SPECIAL START
std::optional<std::vector<uint8_t>> getStringSignal(std::string signalName);
std::optional<int64_t> getInt32Signal(std::string signalName);
std::optional<double> getFloatSignal(std::string signalName);
// SPECIAL END
void acquireLock();
int64_t addDrawingObject(int64_t objectType, double size,
                         double duplicateTolerance, int64_t parentObjectHandle,
                         int64_t maxItemCount,
                         std::optional<std::vector<double>> color = {});
int64_t addDrawingObjectItem(int64_t drawingObjectHandle,
                             std::vector<double> itemData);
void addForce(int64_t shapeHandle, std::vector<double> position,
              std::vector<double> force);
void addForceAndTorque(int64_t shapeHandle,
                       std::optional<std::vector<double>> force = {},
                       std::optional<std::vector<double>> torque = {});
int64_t addGraphCurve(int64_t graphHandle, std::string curveName, int64_t dim,
                      std::vector<int64_t> streamIds,
                      std::vector<double> defaultValues, std::string unitStr,
                      std::optional<int64_t> options = {},
                      std::optional<std::vector<double>> color = {},
                      std::optional<int64_t> curveWidth = {});
int64_t addGraphStream(int64_t graphHandle, std::string streamName,
                       std::string unit, std::optional<int64_t> options = {},
                       std::optional<std::vector<double>> color = {},
                       std::optional<double> cyclicRange = {});
void addItemToCollection(int64_t collectionHandle, int64_t what,
                         int64_t objectHandle, int64_t options);
void addLog(int64_t verbosityLevel, std::string logMessage);
int64_t addParticleObject(int64_t objectType, double size, double density,
                          std::vector<double> params, double lifeTime,
                          int64_t maxItemCount,
                          std::optional<std::vector<double>> color = {});
void addParticleObjectItem(int64_t objectHandle, std::vector<double> itemData);
void addReferencedHandle(int64_t objectHandle, int64_t referencedHandle);
int64_t addScript(int64_t scriptType);
int64_t adjustView(int64_t viewHandleOrIndex,
                   int64_t associatedViewableObjectHandle, int64_t options,
                   std::optional<std::string> viewLabel = {});
int64_t alignShapeBB(int64_t shapeHandle, std::vector<double> pose);
std::tuple<double, double, double>
alphaBetaGammaToYawPitchRoll(double alphaAngle, double betaAngle,
                             double gammaAngle);
int64_t announceSceneContentChange();
void associateScriptWithObject(int64_t scriptHandle, int64_t objectHandle);
int64_t auxiliaryConsoleClose(int64_t consoleHandle);
int64_t
auxiliaryConsoleOpen(std::string title, int64_t maxLines, int64_t mode,
                     std::optional<std::vector<int64_t>> position = {},
                     std::optional<std::vector<int64_t>> size = {},
                     std::optional<std::vector<double>> textColor = {},
                     std::optional<std::vector<double>> backgroundColor = {});
int64_t auxiliaryConsolePrint(int64_t consoleHandle, std::string text);
int64_t auxiliaryConsoleShow(int64_t consoleHandle, bool showState);
void broadcastMsg(json message, std::optional<int64_t> options = {});
std::vector<double> buildIdentityMatrix();
std::vector<double> buildMatrix(std::vector<double> position,
                                std::vector<double> eulerAngles);
std::vector<double> buildMatrixQ(std::vector<double> position,
                                 std::vector<double> quaternion);
std::vector<double> buildPose(std::vector<double> position,
                              std::vector<double> eulerAnglesOrAxis,
                              std::optional<int64_t> mode = {},
                              std::optional<std::vector<double>> axis2 = {});
json callScriptFunction(std::string functionName, int64_t scriptHandle,
                        std::optional<json> inArg = {});
int64_t cameraFitToView(int64_t viewHandleOrIndex,
                        std::optional<std::vector<int64_t>> objectHandles = {},
                        std::optional<int64_t> options = {},
                        std::optional<double> scaling = {});
std::vector<json> changeEntityColor(int64_t entityHandle,
                                    std::vector<double> newColor,
                                    std::optional<int64_t> colorComponent = {});
std::tuple<int64_t, std::vector<int64_t>> checkCollision(int64_t entity1Handle,
                                                         int64_t entity2Handle);
std::tuple<int64_t, std::vector<double>>
checkCollisionEx(int64_t entity1Handle, int64_t entity2Handle);
std::tuple<int64_t, std::vector<double>, std::vector<int64_t>>
checkDistance(int64_t entity1Handle, int64_t entity2Handle,
              std::optional<double> threshold = {});
std::tuple<int64_t, int64_t, int64_t, int64_t>
checkOctreePointOccupancy(int64_t octreeHandle, int64_t options,
                          std::vector<double> points);
std::tuple<int64_t, double, std::vector<double>, int64_t, std::vector<double>>
checkProximitySensor(int64_t sensorHandle, int64_t entityHandle);
std::tuple<int64_t, double, std::vector<double>, int64_t, std::vector<double>>
checkProximitySensorEx(int64_t sensorHandle, int64_t entityHandle, int64_t mode,
                       double threshold, double maxAngle);
std::tuple<int64_t, double, std::vector<double>, std::vector<double>>
checkProximitySensorEx2(int64_t sensorHandle, std::vector<double> vertices,
                        int64_t itemType, int64_t itemCount, int64_t mode,
                        double threshold, double maxAngle);
std::tuple<int64_t, std::vector<double>, std::vector<double>>
checkVisionSensor(int64_t sensorHandle, int64_t entityHandle);
std::vector<double> checkVisionSensorEx(int64_t sensorHandle,
                                        int64_t entityHandle, bool returnImage);
void clearFloatSignal(std::string signalName);
void clearInt32Signal(std::string signalName);
void clearStringSignal(std::string signalName);
int64_t closeScene();
std::vector<uint8_t> combineRgbImages(std::vector<uint8_t> img1,
                                      std::vector<int64_t> img1Res,
                                      std::vector<uint8_t> img2,
                                      std::vector<int64_t> img2Res,
                                      int64_t operation);
int64_t computeMassAndInertia(int64_t shapeHandle, double density);
int64_t convexDecompose(int64_t shapeHandle, int64_t options,
                        std::vector<int64_t> intParams,
                        std::vector<double> floatParams);
std::vector<int64_t> copyPasteObjects(std::vector<int64_t> objectHandles,
                                      std::optional<int64_t> options = {});
std::vector<json> copyTable(std::vector<json> original);
int64_t createCollection(std::optional<int64_t> options = {});
int64_t createDummy(double size);
int64_t createForceSensor(int64_t options, std::vector<int64_t> intParams,
                          std::vector<double> floatParams);
int64_t createHeightfieldShape(int64_t options, double shadingAngle,
                               int64_t xPointCount, int64_t yPointCount,
                               double xSize, std::vector<double> heights);
int64_t createJoint(int64_t jointType, int64_t jointMode, int64_t options,
                    std::optional<std::vector<double>> sizes = {});
int64_t createOctree(double voxelSize, int64_t options, double pointSize);
int64_t createPath(std::vector<double> ctrlPts,
                   std::optional<int64_t> options = {},
                   std::optional<int64_t> subdiv = {},
                   std::optional<double> smoothness = {},
                   std::optional<int64_t> orientationMode = {},
                   std::optional<std::vector<double>> upVector = {});
int64_t createPointCloud(double maxVoxelSize, int64_t maxPtCntPerVoxel,
                         int64_t options, double pointSize);
int64_t createPrimitiveShape(int64_t primitiveType, std::vector<double> sizes,
                             std::optional<int64_t> options = {});
int64_t createProximitySensor(int64_t sensorType, int64_t subType,
                              int64_t options, std::vector<int64_t> intParams,
                              std::vector<double> floatParams);
int64_t createShape(int64_t options, double shadingAngle,
                    std::vector<double> vertices, std::vector<int64_t> indices,
                    std::vector<double> normals,
                    std::vector<double> textureCoordinates,
                    std::vector<uint8_t> texture,
                    std::vector<int64_t> textureResolution);
std::tuple<int64_t, int64_t, std::vector<int64_t>>
createTexture(std::string fileName, int64_t options,
              std::optional<std::vector<double>> planeSizes = {},
              std::optional<std::vector<double>> scalingUV = {},
              std::optional<std::vector<double>> xy_g = {},
              std::optional<int64_t> fixedResolution = {},
              std::optional<std::vector<int64_t>> resolution = {});
int64_t createVisionSensor(int64_t options, std::vector<int64_t> intParams,
                           std::vector<double> floatParams);
void destroyCollection(int64_t collectionHandle);
void destroyGraphCurve(int64_t graphHandle, int64_t curveId);
int64_t duplicateGraphCurveToStatic(int64_t graphHandle, int64_t curveId,
                                    std::optional<std::string> curveName = {});
std::tuple<int64_t, json> executeScriptString(std::string stringToExecute,
                                              int64_t scriptHandle);
void exportMesh(int64_t fileformat, std::string pathAndFilename,
                int64_t options, double scalingFactor,
                std::vector<double> vertices, std::vector<int64_t> indices);
int64_t floatingViewAdd(double posX, double posY, double sizeX, double sizeY,
                        int64_t options);
int64_t floatingViewRemove(int64_t floatingViewHandle);
int64_t generateShapeFromPath(std::vector<double> path,
                              std::vector<double> section,
                              std::optional<int64_t> options = {},
                              std::optional<std::vector<double>> upVector = {});
int64_t generateTextShape(std::string txt,
                          std::optional<std::vector<double>> color = {},
                          std::optional<double> height = {},
                          std::optional<bool> centered = {},
                          std::optional<std::string> alphabetLocation = {});
std::tuple<std::vector<double>, std::vector<double>>
generateTimeOptimalTrajectory(std::vector<double> path,
                              std::vector<double> pathLengths,
                              std::vector<double> minMaxVel,
                              std::vector<double> minMaxAccel,
                              std::optional<int64_t> trajPtSamples = {},
                              std::optional<std::string> boundaryCondition = {},
                              std::optional<double> timeout = {});
std::vector<double>
getAlternateConfigs(std::vector<int64_t> jointHandles,
                    std::vector<double> inputConfig,
                    std::optional<int64_t> tipHandle = {},
                    std::optional<std::vector<double>> lowLimits = {},
                    std::optional<std::vector<double>> ranges = {});
std::vector<std::string> getApiFunc(int64_t scriptHandle, std::string apiWord);
std::string getApiInfo(int64_t scriptHandle, std::string apiWord);
std::vector<double> getArrayParam(int64_t parameter);
double getAutoYieldDelay();
bool getBoolParam(int64_t parameter);
double getClosestPosOnPath(std::vector<double> path,
                           std::vector<double> pathLengths,
                           std::vector<double> absPt);
std::vector<int64_t> getCollectionObjects(int64_t collectionHandle);
double getConfigDistance(std::vector<double> configA,
                         std::vector<double> configB,
                         std::optional<std::vector<double>> metric = {},
                         std::optional<std::vector<int64_t>> types = {});
std::tuple<std::vector<int64_t>, std::vector<double>, std::vector<double>,
           std::vector<double>>
getContactInfo(int64_t dynamicPass, int64_t objectHandle, int64_t index);
std::tuple<std::vector<double>, std::vector<int64_t>>
getDecimatedMesh(std::vector<double> verticesIn, std::vector<int64_t> indicesIn,
                 double decimationPercentage);
bool getEngineBoolParam(int64_t paramId, int64_t objectHandle);
double getEngineFloatParam(int64_t paramId, int64_t objectHandle);
int64_t getEngineInt32Param(int64_t paramId, int64_t objectHandle);
std::vector<double> getEulerAnglesFromMatrix(std::vector<double> matrix);
int64_t getExplicitHandling(int64_t objectHandle);
std::string getExtensionString(int64_t objectHandle, int64_t index,
                               std::optional<std::string> key = {});
double getFloatParam(int64_t parameter);
std::vector<json> getGenesisEvents();
std::tuple<std::string, int64_t, std::vector<double>, std::vector<double>,
           std::vector<double>, std::vector<double>, int64_t, int64_t>
getGraphCurve(int64_t graphHandle, int64_t graphType, int64_t curveIndex);
std::tuple<int64_t, std::vector<double>, std::vector<double>, int64_t>
getGraphInfo(int64_t graphHandle);
int64_t getInt32Param(int64_t parameter);
int64_t getIsRealTimeSimulation();
std::tuple<int64_t, double, double> getJointDependency(int64_t jointHandle);
double getJointForce(int64_t jointHandle);
std::tuple<bool, std::vector<double>> getJointInterval(int64_t objectHandle);
std::tuple<int64_t, int64_t> getJointMode(int64_t jointHandle);
double getJointPosition(int64_t objectHandle);
double getJointTargetForce(int64_t jointHandle);
double getJointTargetPosition(int64_t objectHandle);
double getJointTargetVelocity(int64_t objectHandle);
int64_t getJointType(int64_t objectHandle);
double getJointVelocity(int64_t jointHandle);
std::string getLastInfo();
std::tuple<int64_t, std::vector<double>, std::vector<double>,
           std::vector<double>>
getLightParameters(int64_t lightHandle);
int64_t getLinkDummy(int64_t dummyHandle);
std::vector<std::string> getMatchingPersistentDataTags(std::string pattern);
std::vector<double> getMatrixInverse(std::vector<double> matrix);
int64_t getModelProperty(int64_t objectHandle);
bool getNamedBoolParam(std::string name);
double getNamedFloatParam(std::string name);
int64_t getNamedInt32Param(std::string name);
std::vector<uint8_t> getNamedStringParam(std::string paramName);
int64_t getNavigationMode();
int64_t getObject(std::string path, std::optional<json> options = {});
std::string getObjectAlias(int64_t objectHandle,
                           std::optional<int64_t> options = {});
std::string getObjectAliasRelative(int64_t handle, int64_t baseHandle,
                                   std::optional<int64_t> options = {});
int64_t getObjectChild(int64_t objectHandle, int64_t index);
std::vector<double> getObjectChildPose(int64_t objectHandle);
std::vector<double> getObjectColor(int64_t objectHandle, int64_t index,
                                   int64_t colorComponent);
std::vector<double> getObjectFloatArrayParam(int64_t objectHandle,
                                             int64_t parameterID);
double getObjectFloatParam(int64_t objectHandle, int64_t parameterID);
void getObjectFromUid(int64_t uid, std::optional<json> options = {});
int64_t getObjectInt32Param(int64_t objectHandle, int64_t parameterID);
std::vector<double>
getObjectMatrix(int64_t objectHandle,
                std::optional<int64_t> relativeToObjectHandle = {});
std::vector<double>
getObjectOrientation(int64_t objectHandle,
                     std::optional<int64_t> relativeToObjectHandle = {});
int64_t getObjectParent(int64_t objectHandle);
std::vector<double>
getObjectPose(int64_t objectHandle,
              std::optional<int64_t> relativeToObjectHandle = {});
std::vector<double>
getObjectPosition(int64_t objectHandle,
                  std::optional<int64_t> relativeToObjectHandle = {});
int64_t getObjectProperty(int64_t objectHandle);
std::vector<double>
getObjectQuaternion(int64_t objectHandle,
                    std::optional<int64_t> relativeToObjectHandle = {});
std::vector<int64_t> getObjectSel();
double getObjectSizeFactor(int64_t ObjectHandle);
int64_t getObjectSpecialProperty(int64_t objectHandle);
std::vector<uint8_t> getObjectStringParam(int64_t objectHandle,
                                          int64_t parameterID);
int64_t getObjectType(int64_t objectHandle);
int64_t getObjectUid(int64_t objectHandle);
std::tuple<std::vector<double>, std::vector<double>>
getObjectVelocity(int64_t objectHandle);
int64_t getObjects(int64_t index, int64_t objectType);
std::vector<int64_t> getObjectsInTree(int64_t treeBaseHandle,
                                      std::optional<int64_t> objectType = {},
                                      std::optional<int64_t> options = {});
std::vector<double> getOctreeVoxels(int64_t octreeHandle);
int64_t getPage();
std::vector<double>
getPathInterpolatedConfig(std::vector<double> path,
                          std::vector<double> pathLengths, double t,
                          std::optional<json> method = {},
                          std::optional<std::vector<int64_t>> types = {});
std::tuple<std::vector<double>, double>
getPathLengths(std::vector<double> path, int64_t dof,
               std::optional<std::string> distCallback = {});
std::vector<std::string> getPersistentDataTags();
std::string getPluginInfo(std::string pluginName, int64_t infoType);
std::string getPluginName(int64_t index);
std::tuple<double, int64_t, int64_t, double>
getPointCloudOptions(int64_t pointCloudHandle);
std::vector<double> getPointCloudPoints(int64_t pointCloudHandle);
std::vector<double> getPoseInverse(std::vector<double> pose);
std::tuple<std::vector<double>, std::vector<int64_t>>
getQHull(std::vector<double> verticesIn);
std::vector<double> getQuaternionFromMatrix(std::vector<double> matrix);
double getRandom(std::optional<int64_t> seed = {});
std::vector<int64_t> getReferencedHandles(int64_t objectHandle);
std::tuple<std::vector<double>, double>
getRotationAxis(std::vector<double> matrixStart,
                std::vector<double> matrixGoal);
std::tuple<std::vector<uint8_t>, std::vector<int64_t>>
getScaledImage(std::vector<uint8_t> imageIn, std::vector<int64_t> resolutionIn,
               std::vector<int64_t> desiredResolutionOut, int64_t options);
int64_t getScript(int64_t scriptType, std::optional<int64_t> objectHandle = {},
                  std::optional<std::string> scriptName = {});
json getScriptFunctions(int64_t scriptHandle);
int64_t getScriptInt32Param(int64_t scriptHandle, int64_t parameterID);
std::vector<uint8_t> getScriptStringParam(int64_t scriptHandle,
                                          int64_t parameterID);
bool getSettingBool(std::string key);
double getSettingFloat(std::string key);
int64_t getSettingInt32(std::string key);
std::string getSettingString(std::string key);
std::vector<double> getShapeBB(int64_t shapeHandle);
std::tuple<int64_t, std::vector<double>> getShapeColor(int64_t shapeHandle,
                                                       std::string colorName,
                                                       int64_t colorComponent);
std::tuple<int64_t, int64_t, std::vector<double>>
getShapeGeomInfo(int64_t shapeHandle);
std::tuple<std::vector<double>, std::vector<double>>
getShapeInertia(int64_t shapeHandle);
double getShapeMassAndInertia(int64_t shapeHandle);
std::tuple<std::vector<double>, std::vector<int64_t>, std::vector<double>>
getShapeMesh(int64_t shapeHandle);
int64_t getShapeTextureId(int64_t shapeHandle);
json getShapeViz(int64_t shapeHandle, int64_t itemIndex);
std::string getSignalName(int64_t signalIndex, int64_t signalType);
int64_t getSimulationState();
bool getSimulationStopping();
double getSimulationTime();
double getSimulationTimeStep();
std::tuple<int64_t, std::vector<int64_t>, std::vector<int64_t>>
getSimulatorMessage();
std::string getStackTraceback(std::optional<int64_t> scriptHandle = {});
std::string getStringParam(int64_t parameter);
double getSystemTime();
std::tuple<int64_t, std::vector<int64_t>> getTextureId(std::string textureName);
int64_t getThreadId();
std::vector<std::string> getUserVariables();
std::tuple<std::vector<double>, std::vector<double>>
getVelocity(int64_t shapeHandle);
std::tuple<std::vector<uint8_t>, std::vector<int64_t>>
getVisionSensorDepth(int64_t sensorHandle, std::optional<int64_t> options = {},
                     std::optional<std::vector<int64_t>> pos = {},
                     std::optional<std::vector<int64_t>> size = {});
std::tuple<std::vector<uint8_t>, std::vector<int64_t>>
getVisionSensorImg(int64_t sensorHandle, std::optional<int64_t> options = {},
                   std::optional<double> rgbaCutOff = {},
                   std::optional<std::vector<int64_t>> pos = {},
                   std::optional<std::vector<int64_t>> size = {});
void getVisionSensorRes(int64_t sensorHandle);
int64_t groupShapes(std::vector<int64_t> shapeHandles,
                    std::optional<bool> merge = {});
int64_t handleAddOnScripts(int64_t callType);
int64_t handleChildScripts(int64_t callType);
int64_t handleDynamics(double deltaTime);
int64_t handleEmbeddedScripts(int64_t callType);
void handleGraph(int64_t objectHandle, double simulationTime);
void handleJointMotion();
std::tuple<int64_t, double, std::vector<double>, int64_t, std::vector<double>>
handleProximitySensor(int64_t sensorHandle);
void handleSandboxScript(int64_t callType);
void handleSensingStart();
void handleSimulationStart();
std::tuple<int64_t, std::vector<double>, std::vector<double>>
handleVisionSensor(int64_t sensorHandle);
std::tuple<std::vector<double>, std::vector<int64_t>>
importMesh(int64_t fileformat, std::string pathAndFilename, int64_t options,
           double identicalVerticeTolerance, double scalingFactor);
int64_t importShape(int64_t fileformat, std::string pathAndFilename,
                    int64_t options, double identicalVerticeTolerance,
                    double scalingFactor);
bool initScript(int64_t scriptHandle);
int64_t insertObjectIntoOctree(int64_t octreeHandle, int64_t objectHandle,
                               int64_t options,
                               std::optional<std::vector<double>> color = {},
                               std::optional<int64_t> tag = {});
int64_t
insertObjectIntoPointCloud(int64_t pointCloudHandle, int64_t objectHandle,
                           int64_t options, double gridSize,
                           std::optional<std::vector<double>> color = {},
                           std::optional<double> duplicateTolerance = {});
int64_t
insertPointsIntoPointCloud(int64_t pointCloudHandle, int64_t options,
                           std::vector<double> points,
                           std::optional<std::vector<double>> color = {},
                           std::optional<double> duplicateTolerance = {});
int64_t insertVoxelsIntoOctree(int64_t octreeHandle, int64_t options,
                               std::vector<double> points,
                               std::optional<std::vector<double>> color = {},
                               std::optional<std::vector<int64_t>> tag = {});
std::vector<double> interpolateMatrices(std::vector<double> matrixIn1,
                                        std::vector<double> matrixIn2,
                                        double interpolFactor);
std::vector<double> interpolatePoses(std::vector<double> poseIn1,
                                     std::vector<double> poseIn2,
                                     double interpolFactor);
int64_t intersectPointsWithPointCloud(int64_t pointCloudHandle, int64_t options,
                                      std::vector<double> points,
                                      double tolerance);
int64_t isDeprecated(std::string funcOrConst);
bool isDynamicallyEnabled(int64_t objectHandle);
bool isHandle(int64_t objectHandle);
void launchExecutable(std::string filename,
                      std::optional<std::string> parameters = {},
                      std::optional<int64_t> showStatus = {});
std::tuple<std::vector<uint8_t>, std::vector<int64_t>>
loadImage(int64_t options, std::string filename);
int64_t loadModel(std::string filename);
void loadScene(std::string filename);
std::vector<double> matrixToPose(std::vector<double> matrix);
int64_t moduleEntry(int64_t handle, std::optional<std::string> label = {},
                    std::optional<int64_t> state = {});
std::tuple<std::vector<double>, std::vector<double>, std::vector<double>,
           double>
moveToConfig(int64_t flags, std::vector<double> currentPos,
             std::vector<double> currentVel, std::vector<double> currentAccel,
             std::vector<double> maxVel, std::vector<double> maxAccel,
             std::vector<double> maxJerk, std::vector<double> targetPos,
             std::vector<double> targetVel, std::string callback,
             std::optional<json> auxData = {},
             std::optional<std::vector<bool>> cyclicJoints = {},
             std::optional<double> timeStep = {});
std::tuple<std::vector<double>, double>
moveToPose(int64_t flags, std::vector<double> currentPose,
           std::vector<double> maxVel, std::vector<double> maxAccel,
           std::vector<double> maxJerk, std::vector<double> targetPose,
           std::string callback, std::optional<json> auxData = {},
           std::optional<std::vector<double>> metric = {},
           std::optional<double> timeStep = {});
std::vector<double> multiplyMatrices(std::vector<double> matrixIn1,
                                     std::vector<double> matrixIn2);
std::vector<double> multiplyPoses(std::vector<double> poseIn1,
                                  std::vector<double> poseIn2);
std::vector<double> multiplyVector(std::vector<double> matrix,
                                   std::vector<double> inVectors);
std::vector<uint8_t>
packDoubleTable(std::vector<double> doubleNumbers,
                std::optional<int64_t> startDoubleIndex = {},
                std::optional<int64_t> doubleCount = {});
std::vector<uint8_t> packFloatTable(std::vector<double> floatNumbers,
                                    std::optional<int64_t> startFloatIndex = {},
                                    std::optional<int64_t> floatCount = {});
std::vector<uint8_t> packInt32Table(std::vector<int64_t> int32Numbers,
                                    std::optional<int64_t> startInt32Index = {},
                                    std::optional<int64_t> int32Count = {});
std::vector<uint8_t> packTable(std::vector<json> aTable,
                               std::optional<int64_t> scheme = {});
std::vector<uint8_t>
packUInt16Table(std::vector<int64_t> uint16Numbers,
                std::optional<int64_t> startUint16Index = {},
                std::optional<int64_t> uint16Count = {});
std::vector<uint8_t>
packUInt32Table(std::vector<int64_t> uint32Numbers,
                std::optional<int64_t> startUInt32Index = {},
                std::optional<int64_t> uint32Count = {});
std::vector<uint8_t> packUInt8Table(std::vector<int64_t> uint8Numbers,
                                    std::optional<int64_t> startUint8Index = {},
                                    std::optional<int64_t> uint8count = {});
int64_t pauseSimulation();
std::vector<uint8_t> persistentDataRead(std::string dataTag);
void persistentDataWrite(std::string dataTag, std::vector<uint8_t> dataValue,
                         std::optional<int64_t> options = {});
std::vector<double> poseToMatrix(std::vector<double> pose);
void pushUserEvent(std::string event, int64_t handle, int64_t uid,
                   json eventData, std::optional<int64_t> options = {});
void quitSimulator();
std::vector<uint8_t> readCustomDataBlock(int64_t objectHandle,
                                         std::string tagName);
void readCustomDataBlockEx(int64_t handle, std::string tagName,
                           std::optional<json> options = {});
std::vector<std::string> readCustomDataBlockTags(int64_t objectHandle);
void readCustomTableData(int64_t handle, std::string tagName,
                         std::optional<json> options = {});
std::tuple<int64_t, std::vector<double>, std::vector<double>>
readForceSensor(int64_t objectHandle);
std::tuple<int64_t, double, std::vector<double>, int64_t, std::vector<double>>
readProximitySensor(int64_t sensorHandle);
std::vector<uint8_t> readTexture(int64_t textureId, int64_t options,
                                 std::optional<int64_t> posX = {},
                                 std::optional<int64_t> posY = {},
                                 std::optional<int64_t> sizeX = {},
                                 std::optional<int64_t> sizeY = {});
std::tuple<int64_t, std::vector<double>, std::vector<double>>
readVisionSensor(int64_t sensorHandle);
int64_t refreshDialogs(int64_t refreshDegree);
void releaseLock();
int64_t relocateShapeFrame(int64_t shapeHandle, std::vector<double> pose);
void removeDrawingObject(int64_t drawingObjectHandle);
int64_t removeModel(int64_t objectHandle);
void removeObjects(std::vector<int64_t> objectHandles);
void removeParticleObject(int64_t particleObjectHandle);
int64_t removePointsFromPointCloud(int64_t pointCloudHandle, int64_t options,
                                   std::vector<double> points,
                                   double tolerance);
void removeReferencedObjects(int64_t objectHandle);
void removeScript(int64_t scriptHandle);
int64_t removeVoxelsFromOctree(int64_t octreeHandle, int64_t options,
                               std::vector<double> points);
std::vector<double>
resamplePath(std::vector<double> path, std::vector<double> pathLengths,
             int64_t finalConfigCnt, std::optional<json> method = {},
             std::optional<std::vector<int64_t>> types = {});
void resetDynamicObject(int64_t objectHandle);
void resetGraph(int64_t objectHandle);
void resetProximitySensor(int64_t objectHandle);
void resetVisionSensor(int64_t sensorHandle);
void restoreEntityColor(std::vector<json> originalColorData);
std::vector<double> rotateAroundAxis(std::vector<double> matrixIn,
                                     std::vector<double> axis,
                                     std::vector<double> axisPos, double angle);
int64_t ruckigPos(int64_t dofs, double baseCycleTime, int64_t flags,
                  std::vector<double> currentPosVelAccel,
                  std::vector<double> maxVelAccelJerk,
                  std::vector<int64_t> selection,
                  std::vector<double> targetPosVel);
void ruckigRemove(int64_t handle);
std::tuple<int64_t, std::vector<double>, double> ruckigStep(int64_t handle,
                                                            double cycleTime);
int64_t ruckigVel(int64_t dofs, double baseCycleTime, int64_t flags,
                  std::vector<double> currentPosVelAccel,
                  std::vector<double> maxAccelJerk,
                  std::vector<int64_t> selection,
                  std::vector<double> targetVel);
std::vector<uint8_t> saveImage(std::vector<uint8_t> image,
                               std::vector<int64_t> resolution, int64_t options,
                               std::string filename, int64_t quality);
void saveModel(int64_t modelBaseHandle, std::string filename);
void saveScene(std::string filename);
void scaleObject(int64_t objectHandle, double xScale, double yScale,
                 double zScale, std::optional<int64_t> options = {});
void scaleObjects(std::vector<int64_t> objectHandles, double scalingFactor,
                  bool scalePositionsToo);
int64_t serialCheck(int64_t portHandle);
void serialClose(int64_t portHandle);
int64_t serialOpen(std::string portString, int64_t baudrate);
std::vector<uint8_t>
serialRead(int64_t portHandle, int64_t dataLengthToRead, bool blockingOperation,
           std::optional<std::vector<uint8_t>> closingString = {},
           std::optional<double> timeout = {});
int64_t serialSend(int64_t portHandle, std::vector<uint8_t> data);
void setArrayParam(int64_t parameter, std::vector<double> arrayOfValues);
void setAutoYieldDelay(double dt);
void setBoolParam(int64_t parameter, bool boolState);
void setEngineBoolParam(int64_t paramId, int64_t objectHandle, bool boolParam);
void setEngineFloatParam(int64_t paramId, int64_t objectHandle,
                         double floatParam);
void setEngineInt32Param(int64_t paramId, int64_t objectHandle,
                         int64_t int32Param);
void setExplicitHandling(int64_t objectHandle, int64_t explicitHandlingFlags);
void setFloatParam(int64_t parameter, double floatState);
void setFloatSignal(std::string signalName, double signalValue);
void setGraphStreamTransformation(int64_t graphHandle, int64_t streamId,
                                  int64_t trType,
                                  std::optional<double> mult = {},
                                  std::optional<double> off = {},
                                  std::optional<int64_t> movAvgPeriod = {});
void setGraphStreamValue(int64_t graphHandle, int64_t streamId, double value);
void setInt32Param(int64_t parameter, int64_t intState);
void setInt32Signal(std::string signalName, int64_t signalValue);
void setJointDependency(int64_t jointHandle, int64_t masterJointHandle,
                        double offset, double multCoeff);
void setJointInterval(int64_t objectHandle, bool cyclic,
                      std::vector<double> interval);
void setJointMode(int64_t jointHandle, int64_t jointMode, int64_t options);
void setJointPosition(int64_t objectHandle, double position);
void setJointTargetForce(int64_t objectHandle, double forceOrTorque,
                         std::optional<bool> signedValue = {});
void setJointTargetPosition(
    int64_t objectHandle, double targetPosition,
    std::optional<std::vector<double>> motionParams = {});
void setJointTargetVelocity(
    int64_t objectHandle, double targetVelocity,
    std::optional<std::vector<double>> motionParams = {});
void setLightParameters(int64_t lightHandle, int64_t state,
                        std::vector<double> reserved,
                        std::vector<double> diffusePart,
                        std::vector<double> specularPart);
void setLinkDummy(int64_t dummyHandle, int64_t linkDummyHandle);
void setModelProperty(int64_t objectHandle, int64_t property);
void setNamedBoolParam(std::string name, bool value);
void setNamedFloatParam(std::string name, double value);
void setNamedInt32Param(std::string name, int64_t value);
void setNamedStringParam(std::string paramName,
                         std::vector<uint8_t> stringParam);
void setNavigationMode(int64_t navigationMode);
void setObjectAlias(int64_t objectHandle, std::string objectAlias);
void setObjectChildPose(int64_t objectHandle, std::vector<double> pose);
bool setObjectColor(int64_t objectHandle, int64_t index, int64_t colorComponent,
                    std::vector<double> rgbData);
void setObjectFloatArrayParam(int64_t objectHandle, int64_t parameterID,
                              std::vector<double> params);
void setObjectFloatParam(int64_t objectHandle, int64_t parameterID,
                         double parameter);
void setObjectInt32Param(int64_t objectHandle, int64_t parameterID,
                         int64_t parameter);
void setObjectMatrix(int64_t objectHandle, std::vector<double> matrix,
                     std::optional<int64_t> relativeToObjectHandle = {});
void setObjectOrientation(int64_t objectHandle, std::vector<double> eulerAngles,
                          std::optional<int64_t> relativeToObjectHandle = {});
void setObjectParent(int64_t objectHandle, int64_t parentObjectHandle,
                     std::optional<bool> keepInPlace = {});
void setObjectPose(int64_t objectHandle, std::vector<double> pose,
                   std::optional<int64_t> relativeToObjectHandle = {});
void setObjectPosition(int64_t objectHandle, std::vector<double> position,
                       std::optional<int64_t> relativeToObjectHandle = {});
void setObjectProperty(int64_t objectHandle, int64_t property);
void setObjectQuaternion(int64_t objectHandle, std::vector<double> quaternion,
                         std::optional<int64_t> relativeToObjectHandle = {});
void setObjectSel(std::vector<int64_t> objectHandles);
void setObjectSpecialProperty(int64_t objectHandle, int64_t property);
void setObjectStringParam(int64_t objectHandle, int64_t parameterID,
                          std::vector<uint8_t> parameter);
void setPage(int64_t pageIndex);
void setPluginInfo(std::string pluginName, int64_t infoType, std::string info);
void setPointCloudOptions(int64_t pointCloudHandle, double maxVoxelSize,
                          int64_t maxPtCntPerVoxel, int64_t options,
                          double pointSize);
void setReferencedHandles(int64_t objectHandle,
                          std::vector<int64_t> referencedHandles);
void setScriptInt32Param(int64_t scriptHandle, int64_t parameterID,
                         int64_t parameter);
void setScriptStringParam(int64_t scriptHandle, int64_t parameterID,
                          std::vector<uint8_t> parameter);
void setShapeBB(int64_t shapeHandle, std::vector<double> size);
void setShapeColor(int64_t shapeHandle, std::string colorName,
                   int64_t colorComponent, std::vector<double> rgbData);
void setShapeInertia(int64_t shapeHandle, std::vector<double> inertiaMatrix,
                     std::vector<double> transformationMatrix);
void setShapeMass(int64_t shapeHandle, double mass);
void setShapeMaterial(int64_t shapeHandle, int64_t materialIdOrShapeHandle);
void setShapeTexture(int64_t shapeHandle, int64_t textureId,
                     int64_t mappingMode, int64_t options,
                     std::vector<double> uvScaling,
                     std::optional<std::vector<double>> position = {},
                     std::optional<std::vector<double>> orientation = {});
int64_t setStepping(bool enabled);
void setStringParam(int64_t parameter, std::string stringState);
void setStringSignal(std::string signalName, std::vector<uint8_t> signalValue);
void setVisionSensorImg(int64_t sensorHandle, std::vector<uint8_t> image,
                        std::optional<int64_t> options = {},
                        std::optional<std::vector<int64_t>> pos = {},
                        std::optional<std::vector<int64_t>> size = {});
int64_t startSimulation();
void step();
int64_t stopSimulation();
int64_t subtractObjectFromOctree(int64_t octreeHandle, int64_t objectHandle,
                                 int64_t options);
int64_t subtractObjectFromPointCloud(int64_t pointCloudHandle,
                                     int64_t objectHandle, int64_t options,
                                     double tolerance);
int64_t testCB(int64_t a, std::string cb, int64_t b);
std::tuple<std::string, std::vector<int64_t>, std::vector<int64_t>>
textEditorClose(int64_t handle);
std::tuple<std::string, std::vector<int64_t>, std::vector<int64_t>, bool>
textEditorGetInfo(int64_t handle);
int64_t textEditorOpen(std::string initText, std::string properties);
void textEditorShow(int64_t handle, bool showState);
std::vector<uint8_t> transformBuffer(std::vector<uint8_t> inBuffer,
                                     int64_t inFormat, double multiplier,
                                     double offset, int64_t outFormat);
void transformImage(std::vector<uint8_t> image, std::vector<int64_t> resolution,
                    int64_t options);
std::vector<int64_t> ungroupShape(int64_t shapeHandle);
std::vector<double>
unpackDoubleTable(std::vector<uint8_t> data,
                  std::optional<int64_t> startDoubleIndex = {},
                  std::optional<int64_t> doubleCount = {},
                  std::optional<int64_t> additionalByteOffset = {});
std::vector<double>
unpackFloatTable(std::vector<uint8_t> data,
                 std::optional<int64_t> startFloatIndex = {},
                 std::optional<int64_t> floatCount = {},
                 std::optional<int64_t> additionalByteOffset = {});
std::vector<int64_t>
unpackInt32Table(std::vector<uint8_t> data,
                 std::optional<int64_t> startInt32Index = {},
                 std::optional<int64_t> int32Count = {},
                 std::optional<int64_t> additionalByteOffset = {});
json unpackTable(std::vector<uint8_t> buffer);
std::vector<int64_t>
unpackUInt16Table(std::vector<uint8_t> data,
                  std::optional<int64_t> startUint16Index = {},
                  std::optional<int64_t> uint16Count = {},
                  std::optional<int64_t> additionalByteOffset = {});
std::vector<int64_t>
unpackUInt32Table(std::vector<uint8_t> data,
                  std::optional<int64_t> startUint32Index = {},
                  std::optional<int64_t> uint32Count = {},
                  std::optional<int64_t> additionalByteOffset = {});
std::vector<int64_t>
unpackUInt8Table(std::vector<uint8_t> data,
                 std::optional<int64_t> startUint8Index = {},
                 std::optional<int64_t> uint8count = {});
void visitTree(int64_t rootHandle, std::string visitorFunc,
               std::optional<json> options = {});
double wait(double dt, std::optional<bool> simulationTime = {});
json waitForSignal(std::string sigName);
void writeCustomDataBlock(int64_t objectHandle, std::string tagName,
                          std::vector<uint8_t> data);
void writeCustomDataBlockEx(int64_t handle, std::string tagName,
                            std::string data, std::optional<json> options = {});
void writeCustomTableData(int64_t handle, std::string tagName, json theTable,
                          std::optional<json> options = {});
void writeTexture(int64_t textureId, int64_t options,
                  std::vector<uint8_t> textureData,
                  std::optional<int64_t> posX = {},
                  std::optional<int64_t> posY = {},
                  std::optional<int64_t> sizeX = {},
                  std::optional<int64_t> sizeY = {},
                  std::optional<double> interpol = {});
std::tuple<double, double, double>
yawPitchRollToAlphaBetaGamma(double yawAngle, double pitchAngle,
                             double rollAngle);
void yield();
