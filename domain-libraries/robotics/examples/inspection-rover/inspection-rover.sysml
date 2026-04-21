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

    part def InspectionRover :> RobotPlatform {
        attribute robotName = "inspection-rover";
        attribute criticality = "high";
    }

    part def PatrolMission :> MissionProfile {
        attribute missionName = "facility-inspection";
        attribute missionType = "inspection";
    }

    part def WarehouseEnvironment :> OperatingEnvironment {
        attribute environmentName = "warehouse-floor";
        attribute environmentKind = "indoor-industrial";
    }

    part def DifferentialDrive :> DriveTrain {
        attribute driveName = "differential-drive";
    }

    part def WheelMotorController :> MotorController {
        attribute controllerName = "wheel-motor-controller";
    }

    part def FrontLidar :> LidarSensor {
        attribute sensorName = "front-lidar";
        attribute sensorType = "lidar";
    }

    part def BodyImu :> ImuSensor {
        attribute sensorName = "body-imu";
        attribute sensorType = "imu";
    }

    part def OdometryEstimate :> StateEstimate {
        attribute estimateName = "odometry";
        attribute estimateType = "pose";
    }

    part def VelocityControl :> ControlLoop {
        attribute loopName = "velocity-control";
        attribute controlType = "closed-loop";
    }

    part def VelocityCommand :> MotionCommand {
        attribute commandName = "cmd-vel";
        attribute commandType = "twist";
    }

    part def WheelFeedback :> FeedbackSignal {
        attribute signalName = "wheel-encoder-feedback";
        attribute feedbackKind = "encoder";
    }

    part def NavigationBehavior :> NavigationFunction {
        attribute behaviorName = "waypoint-navigation";
    }

    part def LocalizationBehavior :> LocalizationFunction {
        attribute behaviorName = "localization";
    }

    part def RoverRuntime :> RuntimeNode {
        attribute nodeName = "rover-runtime";
    }

    part def LidarNode :> RosLifecycleNode {
        attribute nodeName = "lidar_node";
    }

    part def LocalizationNode :> RosLifecycleNode {
        attribute nodeName = "localization_node";
    }

    part def NavigationNode :> RosNode {
        attribute nodeName = "navigation_node";
    }

    item def LaserScanMessage :> RosMessageType {
        attribute messageName = "sensor_msgs/LaserScan";
    }

    item def TwistMessage :> RosMessageType {
        attribute messageName = "geometry_msgs/Twist";
    }

    part def ScanTopic :> RosTopic {
        attribute topicName = "/scan";
        attribute messageTypeRef = "sensor_msgs/LaserScan";
    }

    part def CmdVelTopic :> RosTopic {
        attribute topicName = "/cmd_vel";
        attribute messageTypeRef = "geometry_msgs/Twist";
    }

    part def TfTopic :> RosTopic {
        attribute topicName = "/tf";
        attribute messageTypeRef = "tf2_msgs/TFMessage";
    }

    part def SensorQos :> RosQosProfile {
        attribute profileName = "sensor_data";
        attribute reliability = "best_effort";
        attribute durability = "volatile";
        attribute depth = "10";
    }

    part def ControlQos :> RosQosProfile {
        attribute profileName = "control_reliable";
        attribute reliability = "reliable";
        attribute durability = "volatile";
        attribute depth = "10";
    }

    part def ScanPublisher :> RosPublisher {
        attribute publisherName = "lidar_scan_pub";
        attribute publisherNode = "lidar_node";
        attribute topicRef = "/scan";
        attribute messageTypeRef = "sensor_msgs/LaserScan";
        attribute qosProfileRef = "sensor_data";
    }

    part def ScanSubscriber :> RosSubscriber {
        attribute subscriberName = "localization_scan_sub";
        attribute subscriberNode = "localization_node";
        attribute topicRef = "/scan";
        attribute messageTypeRef = "sensor_msgs/LaserScan";
        attribute qosProfileRef = "sensor_data";
    }

    part def CmdVelPublisher :> RosPublisher {
        attribute publisherName = "navigation_cmd_vel_pub";
        attribute publisherNode = "navigation_node";
        attribute topicRef = "/cmd_vel";
        attribute messageTypeRef = "geometry_msgs/Twist";
        attribute qosProfileRef = "control_reliable";
    }

    part def NavigateActionServer :> RosActionServer {
        attribute actionName = "navigate_to_pose";
    }

    part def CostmapParameters :> RosParameter {
        attribute parameterName = "local_costmap";
    }

    part def RoverPackage :> RosPackage {
        attribute packageName = "inspection_rover_bringup";
    }

    part def RoverLaunch :> RosLaunchDescription {
        attribute launchName = "inspection_rover.launch.py";
    }

    part def OdomFrame :> RosTransformFrame {
        attribute frameName = "odom";
        attribute parentFrameRef = "map";
    }

    part def BaseLinkFrame :> RosTransformFrame {
        attribute frameName = "base_link";
        attribute parentFrameRef = "odom";
    }

    part def RoverLifecycle :> LifecycleState {
        attribute phase = "active";
    }

    part def RoverHealth :> HealthMonitor {
        attribute monitorName = "ros2-runtime-health";
        attribute healthState = "ready";
    }

    part def CollisionHazard :> Hazard {
        attribute hazardName = "collision-with-obstacle";
        attribute severity = "high";
    }

    part def EmergencyStop :> SafetyFunction {
        attribute functionName = "emergency-stop";
        attribute category = "protective-stop";
    }

    part def StopMode :> FailSafeMode {
        attribute modeName = "halt";
    }
}
