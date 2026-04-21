package SerializationDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;

    attribute def MediaType;
    attribute def SchemaReference;
    attribute def PackageName;

    item def SerializedPayload specializes MessageType {
        attribute mediaType : MediaType;
        attribute schema : SchemaReference;
    }

    item def JsonPayload specializes SerializedPayload;
    item def XmlPayload specializes SerializedPayload;
    item def ProtobufMessage specializes SerializedPayload {
        attribute packageName : PackageName;
    }
}
