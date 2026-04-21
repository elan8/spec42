package InspectionRoverExample {
    import RoboticsCore::*;
    import RobotStructure::*;
    import RobotPerception::*;
    import RobotActuation::*;
    import RobotControl::*;
    import RobotAutonomy::*;
    import RobotRuntime::*;
    import RobotSafetyAssurance::*;
    import Ros2Overlay::*;

    part def InspectionRover specializes RobotPlatform {
        attribute robotName = "inspection-rover";
        attribute criticality = "high";
    }

    part def PatrolMission specializes MissionProfile {
        attribute missionName = "facility-inspection";
        attribute missionType = "inspection";
    }

    part def WarehouseEnvironment specializes OperatingEnvironment {
        attribute environmentName = "warehouse-floor";
        attribute environmentKind = "indoor-industrial";
    }

    part def DifferentialDrive specializes DriveTrain {
        attribute driveName = "differential-drive";
    }

    part def WheelMotorController specializes MotorController {
        attribute controllerName = "wheel-motor-controller";
    }

    part def FrontLidar specializes LidarSensor {
        attribute sensorName = "front-lidar";
        attribute sensorType = "lidar";
    }

    part def BodyImu specializes ImuSensor {
        attribute sensorName = "body-imu";
        attribute sensorType = "imu";
    }

    part def OdometryEstimate specializes StateEstimate {
        attribute estimateName = "odometry";
        attribute estimateType = "pose";
    }

    part def VelocityControl specializes ControlLoop {
        attribute loopName = "velocity-control";
        attribute controlType = "closed-loop";
    }

    part def VelocityCommand specializes MotionCommand {
        attribute commandName = "cmd-vel";
        attribute commandType = "twist";
    }

    part def WheelFeedback specializes FeedbackSignal {
        attribute signalName = "wheel-encoder-feedback";
        attribute feedbackKind = "encoder";
    }

    part def NavigationBehavior specializes NavigationFunction {
        attribute behaviorName = "waypoint-navigation";
    }

    part def LocalizationBehavior specializes LocalizationFunction {
        attribute behaviorName = "localization";
    }

    part def RoverRuntime specializes RuntimeNode {
        attribute nodeName = "rover-runtime";
    }

    part def LidarNode specializes RosLifecycleNode {
        attribute nodeName = "lidar_node";
    }

    part def LocalizationNode specializes RosLifecycleNode {
        attribute nodeName = "localization_node";
    }

    part def NavigationNode specializes RosNode {
        attribute nodeName = "navigation_node";
    }

    item def LaserScanMessage specializes RosMessageType {
        attribute messageName = "sensor_msgs/LaserScan";
    }

    item def TwistMessage specializes RosMessageType {
        attribute messageName = "geometry_msgs/Twist";
    }

    part def ScanTopic specializes RosTopic {
        attribute topicName = "/scan";
        attribute messageTypeRef = "sensor_msgs/LaserScan";
    }

    part def CmdVelTopic specializes RosTopic {
        attribute topicName = "/cmd_vel";
        attribute messageTypeRef = "geometry_msgs/Twist";
    }

    part def TfTopic specializes RosTopic {
        attribute topicName = "/tf";
        attribute messageTypeRef = "tf2_msgs/TFMessage";
    }

    part def SensorQos specializes RosQosProfile {
        attribute profileName = "sensor_data";
        attribute reliability = "best_effort";
        attribute durability = "volatile";
        attribute depth = "10";
    }

    part def ControlQos specializes RosQosProfile {
        attribute profileName = "control_reliable";
        attribute reliability = "reliable";
        attribute durability = "volatile";
        attribute depth = "10";
    }

    part def ScanPublisher specializes RosPublisher {
        attribute publisherName = "lidar_scan_pub";
        attribute publisherNode = "lidar_node";
        attribute topicRef = "/scan";
        attribute messageTypeRef = "sensor_msgs/LaserScan";
        attribute qosProfileRef = "sensor_data";
    }

    part def ScanSubscriber specializes RosSubscriber {
        attribute subscriberName = "localization_scan_sub";
        attribute subscriberNode = "localization_node";
        attribute topicRef = "/scan";
        attribute messageTypeRef = "sensor_msgs/LaserScan";
        attribute qosProfileRef = "sensor_data";
    }

    part def CmdVelPublisher specializes RosPublisher {
        attribute publisherName = "navigation_cmd_vel_pub";
        attribute publisherNode = "navigation_node";
        attribute topicRef = "/cmd_vel";
        attribute messageTypeRef = "geometry_msgs/Twist";
        attribute qosProfileRef = "control_reliable";
    }

    part def NavigateActionServer specializes RosActionServer {
        attribute actionName = "navigate_to_pose";
    }

    part def CostmapParameters specializes RosParameter {
        attribute parameterName = "local_costmap";
    }

    part def RoverPackage specializes RosPackage {
        attribute packageName = "inspection_rover_bringup";
    }

    part def RoverLaunch specializes RosLaunchDescription {
        attribute launchName = "inspection_rover.launch.py";
    }

    part def OdomFrame specializes RosTransformFrame {
        attribute frameName = "odom";
        attribute parentFrameRef = "map";
    }

    part def BaseLinkFrame specializes RosTransformFrame {
        attribute frameName = "base_link";
        attribute parentFrameRef = "odom";
    }

    part def RoverLifecycle specializes LifecycleState {
        attribute phase = "active";
    }

    part def RoverHealth specializes HealthMonitor {
        attribute monitorName = "ros2-runtime-health";
        attribute healthState = "ready";
    }

    part def CollisionHazard specializes Hazard {
        attribute hazardName = "collision-with-obstacle";
        attribute severity = "high";
    }

    part def EmergencyStop specializes SafetyFunction {
        attribute functionName = "emergency-stop";
        attribute category = "protective-stop";
    }

    part def StopMode specializes FailSafeMode {
        attribute modeName = "halt";
    }
}
