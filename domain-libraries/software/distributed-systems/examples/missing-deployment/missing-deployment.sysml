package MissingDeploymentExample {
    import SoftwareCore::*;
    import DistributedSystems::*;

    interface def BillingApi specializes ApiInterface;

    part def BillingService specializes Service {
        attribute serviceName = "billing";
    }

    part def CustomerService specializes Service {
        attribute serviceName = "customer";
    }

    part def RuntimeNode specializes DeploymentNode {
        attribute nodeName = "runtime-a";
        attribute host = "runtime-a.internal";
    }

    allocate CustomerService to RuntimeNode;
}
