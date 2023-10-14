int64_t addElement(int64_t environmentHandle, int64_t ikGroupHandle,
                   int64_t tipDummyHandle);
std::tuple<int64_t, json, json>
addElementFromScene(int64_t environmentHandle, int64_t ikGroup,
                    int64_t baseHandle, int64_t tipHandle, int64_t targetHandle,
                    int64_t constraints);
std::tuple<std::vector<double>, std::vector<double>>
computeGroupJacobian(int64_t environmentHandle, int64_t ikGroupHandle);
std::tuple<std::vector<double>, std::vector<double>>
computeJacobian(int64_t environmentHandle, int64_t baseObject,
                int64_t lastJoint, int64_t constraints,
                std::vector<double> tipMatrix,
                std::optional<std::vector<double>> targetMatrix = {},
                std::optional<std::vector<double>> constrBaseMatrix = {});
int64_t createDebugOverlay(int64_t environmentHandle, int64_t tipHandle,
                           std::optional<int64_t> baseHandle = {});
int64_t createDummy(int64_t environmentHandle,
                    std::optional<std::string> dummyName = {});
int64_t createEnvironment(std::optional<int64_t> flags = {});
int64_t createGroup(int64_t environmentHandle,
                    std::optional<std::string> ikGroupName = {});
int64_t createJoint(int64_t environmentHandle, int64_t jointType,
                    std::optional<std::string> jointName = {});
bool doesGroupExist(int64_t environmentHandle, std::string ikGroupName);
bool doesObjectExist(int64_t environmentHandle, std::string objectName);
int64_t duplicateEnvironment(int64_t environmentHandle);
void eraseDebugOverlay(int64_t debugObject);
void eraseEnvironment(int64_t environmentHandle);
void eraseObject(int64_t environmentHandle, int64_t objectHandle);
std::vector<double>
findConfig(int64_t environmentHandle, int64_t ikGroupHandle,
           std::vector<int64_t> jointHandles,
           std::optional<double> thresholdDist = {},
           std::optional<double> maxTime = {},
           std::optional<std::vector<double>> metric = {},
           std::optional<std::string> validationCallback = {},
           std::optional<json> auxData = {});
std::vector<double>
generatePath(int64_t environmentHandle, int64_t ikGroupHandle,
             std::vector<int64_t> jointHandles, int64_t tipHandle,
             int64_t pathPointCount,
             std::optional<std::string> validationCallback = {},
             std::optional<json> auxData = {});
std::vector<double>
getAlternateConfigs(int64_t environmentHandle,
                    std::vector<int64_t> jointHandles,
                    std::optional<std::vector<double>> lowLimits = {},
                    std::optional<std::vector<double>> ranges = {});
std::tuple<int64_t, int64_t> getElementBase(int64_t environmentHandle,
                                            int64_t ikGroupHandle,
                                            int64_t elementHandle);
int64_t getElementConstraints(int64_t environmentHandle, int64_t ikGroupHandle,
                              int64_t elementHandle);
int64_t getElementFlags(int64_t environmentHandle, int64_t ikGroupHandle,
                        int64_t elementHandle);
std::vector<double> getElementPrecision(int64_t environmentHandle,
                                        int64_t ikGroupHandle,
                                        int64_t elementHandle);
std::vector<double> getElementWeights(int64_t environmentHandle,
                                      int64_t ikGroupHandle,
                                      int64_t elementHandle);
std::string getFailureDescription(int64_t reason);
std::tuple<int64_t, double, int64_t>
getGroupCalculation(int64_t environmentHandle, int64_t ikGroupHandle);
int64_t getGroupFlags(int64_t environmentHandle, int64_t ikGroupHandle);
int64_t getGroupHandle(int64_t environmentHandle, std::string ikGroupName);
std::tuple<std::vector<int64_t>, std::vector<double>>
getGroupJointLimitHits(int64_t environmentHandle, int64_t ikGroupHandle);
std::vector<int64_t> getGroupJoints(int64_t environmentHandle,
                                    int64_t ikGroupHandle);
std::tuple<int64_t, double, double>
getJointDependency(int64_t environmentHandle, int64_t jointHandle);
std::tuple<bool, std::vector<double>>
getJointInterval(int64_t environmentHandle, int64_t jointHandle);
double getJointLimitMargin(int64_t environmentHandle, int64_t jointHandle);
std::vector<double> getJointMatrix(int64_t environmentHandle,
                                   int64_t jointHandle);
double getJointMaxStepSize(int64_t environmentHandle, int64_t jointHandle);
int64_t getJointMode(int64_t environmentHandle, int64_t jointHandle);
double getJointPosition(int64_t environmentHandle, int64_t jointHandle);
double getJointScrewLead(int64_t environmentHandle, int64_t jointHandle);
std::tuple<std::vector<double>, std::vector<double>, std::vector<double>>
getJointTransformation(int64_t environmentHandle, int64_t jointHandle);
int64_t getJointType(int64_t environmentHandle, int64_t jointHandle);
double getJointWeight(int64_t environmentHandle, int64_t jointHandle);
int64_t getObjectHandle(int64_t environmentHandle, std::string objectName);
std::vector<double>
getObjectMatrix(int64_t environmentHandle, int64_t objectHandle,
                std::optional<int64_t> relativeToObjectHandle = {});
int64_t getObjectParent(int64_t environmentHandle, int64_t objectHandle);
std::vector<double>
getObjectPose(int64_t environmentHandle, int64_t objectHandle,
              std::optional<int64_t> relativeToObjectHandle = {});
std::tuple<std::vector<double>, std::vector<double>, std::vector<double>>
getObjectTransformation(int64_t environmentHandle, int64_t objectHandle,
                        std::optional<int64_t> relativeToObjectHandle = {});
int64_t getObjectType(int64_t environmentHandle, int64_t objectHandle);
std::tuple<int64_t, std::string, bool, int64_t>
getObjects(int64_t environmentHandle, int64_t index);
int64_t getTargetDummy(int64_t environmentHandle, int64_t dummyHandle);
std::tuple<int64_t, int64_t, std::vector<double>>
handleGroup(int64_t environmentHandle, int64_t ikGroup,
            std::optional<json> options = {});
std::tuple<int64_t, int64_t, std::vector<double>>
handleGroups(int64_t environmentHandle, std::vector<int64_t> ikGroups,
             std::optional<json> options = {});
void load(int64_t environmentHandle, std::string data);
std::string save(int64_t environmentHandle);
void setElementBase(int64_t environmentHandle, int64_t ikGroupHandle,
                    int64_t elementHandle, int64_t baseHandle,
                    std::optional<int64_t> constraintsBaseHandle = {});
void setElementConstraints(int64_t environmentHandle, int64_t ikGroupHandle,
                           int64_t elementHandle, int64_t constraints);
void setElementFlags(int64_t environmentHandle, int64_t ikGroupHandle,
                     int64_t elementHandle, int64_t flags);
void setElementPrecision(int64_t environmentHandle, int64_t ikGroupHandle,
                         int64_t elementHandle, std::vector<double> precision);
void setElementWeights(int64_t environmentHandle, int64_t ikGroupHandle,
                       int64_t elementHandle, std::vector<double> weights);
void setGroupCalculation(int64_t environmentHandle, int64_t ikGroupHandle,
                         int64_t method, double damping, int64_t maxIterations);
void setGroupFlags(int64_t environmentHandle, int64_t ikGroupHandle,
                   int64_t flags);
void setJointDependency(int64_t environmentHandle, int64_t jointHandle,
                        int64_t masterJointHandle,
                        std::optional<double> offset = {},
                        std::optional<double> mult = {},
                        std::optional<std::string> callback = {});
void setJointInterval(int64_t environmentHandle, int64_t jointHandle,
                      bool cyclic,
                      std::optional<std::vector<double>> interval = {});
void setJointLimitMargin(int64_t environmentHandle, int64_t jointHandle,
                         double margin);
void setJointMaxStepSize(int64_t environmentHandle, int64_t jointHandle,
                         double stepSize);
void setJointMode(int64_t environmentHandle, int64_t jointHandle,
                  int64_t jointMode);
void setJointPosition(int64_t environmentHandle, int64_t jointHandle,
                      double position);
void setJointScrewLead(int64_t environmentHandle, int64_t jointHandle,
                       double lead);
void setJointWeight(int64_t environmentHandle, int64_t jointHandle,
                    double weight);
void setObjectMatrix(int64_t environmentHandle, int64_t objectHandle,
                     std::vector<double> matrix,
                     std::optional<int64_t> relativeToObjectHandle = {});
void setObjectParent(int64_t environmentHandle, int64_t objectHandle,
                     int64_t parentObjectHandle,
                     std::optional<bool> keepInPlace = {});
void setObjectPose(int64_t environmentHandle, int64_t objectHandle,
                   std::vector<double> pose,
                   std::optional<int64_t> relativeToObjectHandle = {});
void setObjectTransformation(
    int64_t environmentHandle, int64_t objectHandle,
    std::vector<double> position, std::vector<double> eulerOrQuaternion,
    std::optional<int64_t> relativeToObjectHandle = {});
void setSphericalJointMatrix(int64_t environmentHandle, int64_t jointHandle,
                             std::vector<double> matrix);
void setSphericalJointRotation(int64_t environmentHandle, int64_t jointHandle,
                               std::vector<double> eulerOrQuaternion);
void setTargetDummy(int64_t environmentHandle, int64_t dummyHandle,
                    int64_t targetDummyHandle);
void syncFromSim(int64_t environmentHandle, std::vector<int64_t> ikGroups);
void syncToSim(int64_t environmentHandle, std::vector<int64_t> ikGroups);
