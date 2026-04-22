package KubernetesDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;

    attribute def ImageName;
    attribute def ReplicaCount;
    attribute def NamespaceName;

    part def KubernetesCluster :> DeploymentNode {
        attribute clusterName : Name;
    }

    part def KubernetesNamespace {
        attribute namespaceName : NamespaceName;
    }

    part def KubernetesPod :> ServiceInstance {
        attribute image : ImageName;
    }

    part def KubernetesDeployment :> DeployableComponent {
        attribute deploymentName : Name;
        attribute replicas : ReplicaCount;
        attribute image : ImageName;
    }

    part def KubernetesService {
        attribute serviceName : Name;
        attribute port : PortNumber;
    }
}
