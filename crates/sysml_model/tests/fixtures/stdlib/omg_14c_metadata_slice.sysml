package FMEAMetadata {
  private import Metaobjects::SemanticMetadata;

  metadata def FMEARequirementMetadata :> SemanticMetadata {
    :>> annotatedElement : SysML::RequirementUsage;
    :>> baseType = fmeaRequirements meta SysML::Usage;
  }

  metadata def CausationMetadata :> SemanticMetadata {
    :>> annotatedElement : SysML::Usage;
    :>> baseType = causations meta SysML::Usage;
  }
}
