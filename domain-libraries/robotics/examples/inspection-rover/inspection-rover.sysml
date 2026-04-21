package InspectionRoverExample {
    import RoboticsCore::*;
    import RobotRuntime::*;
    import RobotStructure::*;
    import Ros2Overlay::*;

    part def InspectionRover :> RobotPlatform {
        attribute :>> name = "inspection_rover";
        attribute :>> criticality = high;
    }

    part def RoverSystem :> InspectionRover {
        part runtime : RoverRuntimeArchitecture;
        part deployment : RoverDeployment;
        part traceability : RoverTraceability;
    }

    part def RoverRuntimeArchitecture {
        part lidarRuntime : LidarNode;
        part localizationRuntime : LocalizationNode;
        part motionRuntime : MotionControllerNode;

        connect lidarRuntime.scanPubPort to localizationRuntime.scanSubPort;
        connect localizationRuntime.cmdVelPubPort to motionRuntime.cmdVelSubPort;
    }

    part def RoverRuntimeNode :> RosLifecycleNode {
        attribute :>> packageRef = "inspection_rover_bringup";
        attribute :>> namespaceRef = "/inspection_rover";
        attribute :>> launchGroupName = "core_runtime";
    }

    part def RoverCoreRuntimeNode :> RosRuntimeComponent {
        attribute :>> packageRef = "inspection_rover_bringup";
        attribute :>> namespaceRef = "/inspection_rover";
        attribute :>> launchGroupName = "core_runtime";
    }

    part def LidarNode :> RoverRuntimeNode {
        attribute :>> name = "lidar_node";
        attribute :>> executableName = "lidar_node";

        port scanPubPort : RosPublishPort {
            attribute :>> name = "scan_pub";
            attribute :>> ownerNodeRef = "lidar_node";
            attribute :>> topicRef = "/scan";
            attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
            attribute :>> qosProfileRef = "sensor_data";
        }
    }

    part def LocalizationNode :> RoverRuntimeNode {
        attribute :>> name = "localization_node";
        attribute :>> executableName = "localization_node";

        port scanSubPort : RosSubscribePort {
            attribute :>> name = "scan_sub";
            attribute :>> ownerNodeRef = "localization_node";
            attribute :>> topicRef = "/scan";
            attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
            attribute :>> qosProfileRef = "sensor_data";
        }

        port cmdVelPubPort : RosPublishPort {
            attribute :>> name = "cmd_vel_pub";
            attribute :>> ownerNodeRef = "localization_node";
            attribute :>> topicRef = "/cmd_vel";
            attribute :>> messageTypeRef = "geometry_msgs/Twist";
            attribute :>> qosProfileRef = "control_reliable";
        }
    }

    part def MotionControllerNode :> RoverCoreRuntimeNode {
        attribute :>> name = "motion_controller_node";
        attribute :>> executableName = "motion_controller_node";

        port cmdVelSubPort : RosSubscribePort {
            attribute :>> name = "cmd_vel_sub";
            attribute :>> ownerNodeRef = "motion_controller_node";
            attribute :>> topicRef = "/cmd_vel";
            attribute :>> messageTypeRef = "geometry_msgs/Twist";
            attribute :>> qosProfileRef = "control_reliable";
        }
    }

    item def LaserScanMessage :> RosMessageType {
        attribute :>> name = "sensor_msgs/LaserScan";
    }

    item def TwistMessage :> RosMessageType {
        attribute :>> name = "geometry_msgs/Twist";
    }

    item def SetRuntimeModeService :> RosServiceType {
        attribute :>> name = "inspection_msgs/SetRuntimeMode";
    }

    item def NavigateToPointAction :> RosActionType {
        attribute :>> name = "inspection_msgs/NavigateToPoint";
    }

    part def ScanTopic :> RosTopic {
        attribute :>> name = "/scan";
        attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
    }

    part def CmdVelTopic :> RosTopic {
        attribute :>> name = "/cmd_vel";
        attribute :>> messageTypeRef = "geometry_msgs/Twist";
    }

    part def SensorQos :> RosQosProfile {
        attribute :>> name = "sensor_data";
        attribute :>> reliability = best_effort;
    }

    part def ControlQos :> RosQosProfile {
        attribute :>> name = "control_reliable";
        attribute :>> reliability = reliable;
    }

    part def ScanPublisher :> RosPublisher {
        attribute :>> name = "lidar_scan_pub";
        attribute :>> publisherNode = "lidar_node";
        attribute :>> topicRef = "/scan";
        attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
        attribute :>> qosProfileRef = "sensor_data";
    }

    part def ScanSubscriber :> RosSubscriber {
        attribute :>> name = "localization_scan_sub";
        attribute :>> subscriberNode = "localization_node";
        attribute :>> topicRef = "/scan";
        attribute :>> messageTypeRef = "sensor_msgs/LaserScan";
        attribute :>> qosProfileRef = "sensor_data";
    }

    part def CmdVelPublisher :> RosPublisher {
        attribute :>> name = "navigation_cmd_vel_pub";
        attribute :>> publisherNode = "localization_node";
        attribute :>> topicRef = "/cmd_vel";
        attribute :>> messageTypeRef = "geometry_msgs/Twist";
        attribute :>> qosProfileRef = "control_reliable";
    }

    part def SetRuntimeModeServer :> RosService {
        attribute :>> name = "set_runtime_mode";
        attribute :>> serviceTypeRef = "inspection_msgs/SetRuntimeMode";
    }

    part def NavigateToPointServer :> RosActionServer {
        attribute :>> name = "navigate_to_point";
        attribute :>> actionTypeRef = "inspection_msgs/NavigateToPoint";
    }

    part def LocalizationRateParam :> RosParameter {
        attribute :>> name = "localization_rate_hz";
        attribute :>> ownerNodeRef = "localization_node";
        attribute :>> namespaceRef = "/inspection_rover/localization";
        attribute :>> parameterType = float;
        attribute :>> defaultFloat = 20.0;
    }

    part def UseSimTimeParam :> RosParameter {
        attribute :>> name = "use_sim_time";
        attribute :>> ownerNodeRef = "localization_node";
        attribute :>> namespaceRef = "/inspection_rover";
        attribute :>> parameterType = bool;
        attribute :>> defaultBool = false;
    }

    part def OdomFrame :> RosTransformFrame {
        attribute :>> name = "odom";
        attribute :>> parentFrameRef = "map";
    }

    part def BaseLinkFrame :> RosTransformFrame {
        attribute :>> name = "base_link";
        attribute :>> parentFrameRef = "odom";
    }

    part def RoverLifecycle :> LifecycleState {
        attribute :>> phase = active;
    }

    part def RoverPackage :> RosPackage {
        attribute :>> name = "inspection_rover_bringup";
    }

    part def RoverLaunch :> RosLaunchDescription {
        attribute :>> name = "inspection_rover.launch.py";
        attribute :>> launchGroupName = "core_runtime";
        attribute :>> packageRef = "inspection_rover_bringup";
    }

    part def RoverDeploymentUnit :> RosDeploymentUnit {
        attribute :>> packageRef = "inspection_rover_bringup";
    }

    part def LidarDeployment :> RoverDeploymentUnit {
        attribute :>> name = "lidar_deployment";
        attribute :>> executableName = "lidar_node";
        attribute :>> nodeRef = "lidar_node";
    }

    part def LocalizationDeployment :> RoverDeploymentUnit {
        attribute :>> name = "localization_deployment";
        attribute :>> executableName = "localization_node";
        attribute :>> nodeRef = "localization_node";
    }

    part def MotionControllerDeployment :> RoverDeploymentUnit {
        attribute :>> name = "motion_controller_deployment";
        attribute :>> executableName = "motion_controller_node";
        attribute :>> nodeRef = "motion_controller_node";
    }

    part def RoverDeployment {
        part roverPackage : RoverPackage;
        part roverLaunch : RoverLaunch;
        part lidarDeploy : LidarDeployment;
        part localizationDeploy : LocalizationDeployment;
        part motionDeploy : MotionControllerDeployment;
    }

    part def ScanArtifact :> RosGenerationArtifact {
        attribute :>> name = "scan_message";
        attribute :>> artifactKind = interface_msg;
        attribute :>> artifactPath = "interfaces/msg/LaserScan.msg";
    }

    part def RuntimePackageManifest :> RosGenerationArtifact {
        attribute :>> name = "runtime_package_manifest";
        attribute :>> artifactKind = package_manifest;
        attribute :>> artifactPath = "inspection_rover_bringup/package.xml";
    }

    part def RuntimeLaunchArtifact :> RosGenerationArtifact {
        attribute :>> name = "runtime_launch";
        attribute :>> artifactKind = launch_file;
        attribute :>> artifactPath = "inspection_rover_bringup/launch/inspection_rover.launch.py";
    }

    part def RuntimeParamsArtifact :> RosGenerationArtifact {
        attribute :>> name = "runtime_params";
        attribute :>> artifactKind = parameter_file;
        attribute :>> artifactPath = "inspection_rover_bringup/config/runtime.yaml";
    }

    part def TraceScanTopic :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.topic.scan";
        attribute :>> artifactPath = "interfaces/msg/LaserScan.msg";
    }

    part def TraceRuntimeLaunch :> RosTraceabilityEntry {
        attribute :>> modelElementId = "ros.launch.runtime";
        attribute :>> artifactPath = "inspection_rover_bringup/launch/inspection_rover.launch.py";
    }

    part def RoverTraceability {
        part scanArtifact : ScanArtifact;
        part packageManifestArtifact : RuntimePackageManifest;
        part launchArtifact : RuntimeLaunchArtifact;
        part paramsArtifact : RuntimeParamsArtifact;
        part traceScan : TraceScanTopic;
        part traceLaunch : TraceRuntimeLaunch;
    }
}
