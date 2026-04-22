package Ros2Overlay {
    import RoboticsCore::*;
    import RobotRuntime::*;

    attribute def TopicName;
    attribute def PackageName;
    attribute def FrameName;
    attribute def MessageTypeRef;
    attribute def ServiceTypeRef;
    attribute def ActionTypeRef;
    attribute def NodeRef;
    attribute def PortRef;
    attribute def EndpointRef;
    attribute def TopicRef;
    attribute def QosProfileRef;
    attribute def ParentFrameRef;
    attribute def PackageRef;
    attribute def ExecutableName;
    attribute def NamespaceRef;
    attribute def LaunchGroupName;
    attribute def ParameterTypeKind;
    attribute def IntegerValue;
    attribute def FloatValue;
    attribute def BooleanValue;
    attribute def TextValue;
    attribute def ArtifactKind;
    attribute def ArtifactPath;
    attribute def ModelElementId;
    attribute def ReliabilityPolicy;
    attribute def DurabilityPolicy;
    attribute def QueueDepth;
    attribute def ProfileName;
    attribute def LaunchModeKind;
    attribute def IncludeRef;
    attribute def ConditionExpr;
    attribute def SourceName;
    attribute def TargetName;
    attribute def FrameRef;
    attribute def FrameAuthorityKind;
    attribute def ManagedNodeOrder;
    attribute def TargetLifecycleState;
    attribute def EnclaveRef;
    attribute def SecurityPolicyRef;
    attribute def KeystorePathRef;
    attribute def DependencyKind;
    attribute def DependencyPackageRef;
    attribute def ParameterFileRef;

    part def RosNode :> RuntimeNode;
    part def RosRuntimeComponent :> RosNode {
        attribute packageRef : PackageRef;
        attribute executableName : ExecutableName;
        attribute namespaceRef : NamespaceRef = "/";
        attribute launchGroupName : LaunchGroupName = "default";
    }

    part def RosTopic {
        attribute name : TopicName;
        attribute messageTypeRef : MessageTypeRef;
    }

    part def RosPort {
        attribute name : Name;
    }

    part def RosEndpointPort :> RosPort {
        attribute ownerNodeRef : NodeRef;
        attribute topicRef : TopicRef;
        attribute messageTypeRef : MessageTypeRef;
        attribute serviceTypeRef : ServiceTypeRef;
        attribute actionTypeRef : ActionTypeRef;
        attribute qosProfileRef : QosProfileRef;
    }

    part def RosPublishPort :> RosEndpointPort;
    part def RosSubscribePort :> RosEndpointPort;
    part def RosServiceServerPort :> RosEndpointPort;
    part def RosServiceClientPort :> RosEndpointPort;
    part def RosActionServerPort :> RosEndpointPort;
    part def RosActionClientPort :> RosEndpointPort;

    part def RosConnection {
        attribute name : Name;
        attribute sourcePortRef : PortRef;
        attribute targetPortRef : PortRef;
        attribute topicRef : TopicRef;
    }

    part def RosPublisher {
        attribute name : Name;
        attribute publisherNode : NodeRef;
        attribute topicRef : TopicRef;
        attribute messageTypeRef : MessageTypeRef;
        attribute qosProfileRef : QosProfileRef;
    }

    part def RosSubscriber {
        attribute name : Name;
        attribute subscriberNode : NodeRef;
        attribute topicRef : TopicRef;
        attribute messageTypeRef : MessageTypeRef;
        attribute qosProfileRef : QosProfileRef;
    }

    part def RosService {
        attribute name : Name;
        attribute serviceTypeRef : ServiceTypeRef;
    }

    part def RosClient {
        attribute name : Name;
        attribute serviceTypeRef : ServiceTypeRef;
    }

    part def RosActionServer {
        attribute name : Name;
        attribute actionTypeRef : ActionTypeRef;
    }

    part def RosActionClient {
        attribute name : Name;
        attribute actionTypeRef : ActionTypeRef;
    }

    part def RosParameter {
        attribute name : Name;
        attribute ownerNodeRef : NodeRef;
        attribute namespaceRef : NamespaceRef;
        attribute parameterType : ParameterTypeKind;
        attribute defaultInt : IntegerValue;
        attribute defaultFloat : FloatValue;
        attribute defaultBool : BooleanValue;
        attribute defaultText : TextValue;
    }

    part def RosLifecycleNode :> RosRuntimeComponent;

    item def RosMessageType {
        attribute name : Name;
    }
    item def RosServiceType {
        attribute name : Name;
    }
    item def RosActionType {
        attribute name : Name;
    }

    part def RosPackage {
        attribute name : PackageName;
    }

    part def RosLaunchDescription {
        attribute name : Name;
        attribute launchGroupName : LaunchGroupName;
        attribute packageRef : PackageRef;
        attribute launchMode : LaunchModeKind;
    }

    part def RosLaunchInclude {
        attribute name : Name;
        attribute includeRef : IncludeRef;
        attribute conditionExpr : ConditionExpr;
    }

    part def RosLaunchGroup {
        attribute name : Name;
        attribute conditionExpr : ConditionExpr;
    }

    part def RosRemapRule {
        attribute name : Name;
        attribute sourceName : SourceName;
        attribute targetName : TargetName;
        attribute ownerNodeRef : NodeRef;
    }

    part def RosParameterProfile {
        attribute name : ProfileName;
        attribute parameterFileRef : ParameterFileRef;
        attribute ownerNodeRef : NodeRef;
    }

    part def RosTfProducer {
        attribute name : Name;
        attribute sourceFrameRef : FrameRef;
        attribute targetFrameRef : FrameRef;
        attribute authorityKind : FrameAuthorityKind;
        attribute ownerNodeRef : NodeRef;
    }

    part def RosLifecycleManagerConfig {
        attribute name : Name;
        attribute managerNodeRef : NodeRef;
        attribute managedNodeOrder : ManagedNodeOrder;
        attribute targetState : TargetLifecycleState;
    }

    part def RosSecurityProfile {
        attribute name : Name;
        attribute ownerNodeRef : NodeRef;
        attribute enclaveRef : EnclaveRef;
        attribute securityPolicyRef : SecurityPolicyRef;
        attribute keystorePathRef : KeystorePathRef;
    }

    part def RosManifestDependency {
        attribute name : Name;
        attribute ownerPackageRef : PackageRef;
        attribute dependencyKind : DependencyKind;
        attribute dependencyPackageRef : DependencyPackageRef;
    }

    part def RosDeploymentUnit {
        attribute name : Name;
        attribute packageRef : PackageRef;
        attribute executableName : ExecutableName;
        attribute nodeRef : NodeRef;
    }

    part def RosTransformFrame {
        attribute name : FrameName;
        attribute parentFrameRef : ParentFrameRef;
    }

    part def RosQosProfile {
        attribute name : Name;
        attribute reliability : ReliabilityPolicy;
        attribute durability : DurabilityPolicy = volatile;
        attribute depth : QueueDepth = 10;
    }

    part def RosGenerationArtifact {
        attribute name : Name;
        attribute artifactKind : ArtifactKind;
        attribute artifactPath : ArtifactPath;
    }

    part def RosTraceabilityEntry {
        attribute modelElementId : ModelElementId;
        attribute artifactPath : ArtifactPath;
    }
}
