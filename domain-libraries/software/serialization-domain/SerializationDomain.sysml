package SerializationDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;

    attribute def MediaType;
    attribute def SchemaReference;
    attribute def PackageName;

    item def SerializedPayload :> MessageType {
        attribute mediaType : MediaType;
        attribute schema : SchemaReference;
    }

    item def JsonPayload :> SerializedPayload;
    item def XmlPayload :> SerializedPayload;
    item def ProtobufMessage :> SerializedPayload {
        attribute packageName : PackageName;
    }
}
