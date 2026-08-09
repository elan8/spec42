# META
~~~ini
description=Standard Library: Domain Libraries/Cause and Effect/CauseAndEffect
type=file
~~~
# SOURCE
~~~sysml
standard library package CauseAndEffect {
	doc /* This package provides language-extension metadata for cause-effect modeling. */
	
	public import CausationConnections::*;
	private import ScalarValues::*;
	private import Metaobjects::SemanticMetadata;

	metadata def <cause> CauseMetadata :> SemanticMetadata {
		doc
		/*
		 * CauseMetadata identifies a usage as being a cause occurrence.
		 * It is intended to be used to tag the cause ends of a Multicausation.
		 */
		 
		ref :>> annotatedElement : SysML::Usage;
		ref :>> baseType = causes as SysML::Usage;
	}
	
	metadata def <effect> EffectMetadata :> SemanticMetadata {
		doc
		/*
		 * EffectMetadata identifies a usage as being an effect occurrence.
		 * It is intended to be used to tag the effect ends of a Multicausation.
		 */
		 
		ref :>> annotatedElement : SysML::Usage;
		ref :>> baseType = effects as SysML::Usage;
	}
	
	metadata def CausationMetadata {
		doc
		/*
		 * CausationMetadata allows for the specification of additional metadata about
		 * a cause-effect connection definition or usage.
		 */
		 
		ref :> annotatedElement : SysML::ConnectionDefinition;
		ref :> annotatedElement : SysML::ConnectionUsage;
		
		attribute isNecessary : Boolean default false {
			doc 
			/* 
			 * Whether all the causes are necessary for all the effects to occur.
			 * If this is false (the default), then some or all of the effects may 
			 * still have occurred even if some of the causes did not.
			 */
		}
		
		attribute isSufficient : Boolean default false {
			doc
			/*
			 * Whether the causes were sufficient for all the effects to occur.
			 * If this is false (the default), then it may be the case that some
			 * other occurrences were also necessary for some or all of the effects
			 * to have occurred.
			 */
		}
		
		attribute probability : Real[0..1] {
			doc /* The probability that the causes will actually result in effects occurring. */
		}	
	}
	
	metadata def <multicausation> MulticausationSemanticMetadata :> CausationMetadata, SemanticMetadata {
		doc
		/*
		 * MulticausationMetadata is SemanticMetadata for a Multicausation connection.
		 */
		 
		ref :>> baseType = multicausations meta SysML::Usage;
	}
	
	metadata def <causation> CausationSemanticMetadadata :> CausationMetadata, SemanticMetadata {
		doc
		/*
		 * CausationMetadata is SemanticMetadata for a Causation connection.
		 */
		 
		ref :>> baseType = causations meta SysML::Usage;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionDefinition'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionDefinition'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,Eq,Ident,KwAs,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,Eq,Ident,KwAs,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,KwDefault,KwFalse,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,KwDefault,KwFalse,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'CauseAndEffect'
    (documentation)
    (import_decl public 'CausationConnections::*')
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Metaobjects::SemanticMetadata')
    (metadata_def 'CauseMetadata' :> 'SemanticMetadata'
      (documentation)
      (ref_usage ref :>> 'annotatedElement' : 'SysML::Usage')
      (ref_usage ref :>> 'baseType' value))
    (metadata_def 'EffectMetadata' :> 'SemanticMetadata'
      (documentation)
      (ref_usage ref :>> 'annotatedElement' : 'SysML::Usage')
      (ref_usage ref :>> 'baseType' value))
    (metadata_def 'CausationMetadata'
      (documentation)
      (ref_usage ref :> 'annotatedElement' : 'SysML::ConnectionDefinition')
      (ref_usage ref :> 'annotatedElement' : 'SysML::ConnectionUsage')
      (attribute_usage 'isNecessary' : 'Boolean' value
        (documentation))
      (attribute_usage 'isSufficient' : 'Boolean' value
        (documentation))
      (attribute_usage 'probability' : 'Real' multiplicity
        (documentation)))
    (metadata_def 'MulticausationSemanticMetadata' :> 'CausationMetadata', 'SemanticMetadata'
      (documentation)
      (ref_usage ref :>> 'baseType' value))
    (metadata_def 'CausationSemanticMetadadata' :> 'CausationMetadata', 'SemanticMetadata'
      (documentation)
      (ref_usage ref :>> 'baseType' value))))
~~~
# FORMAT
~~~sysml
standard library package CauseAndEffect {
    doc /* This package provides language-extension metadata for cause-effect modeling. */

    public import CausationConnections::*;
    private import ScalarValues::*;
    private import Metaobjects::SemanticMetadata;

    metadata def <cause> CauseMetadata :> SemanticMetadata {
        doc /*
		 * CauseMetadata identifies a usage as being a cause occurrence.
		 * It is intended to be used to tag the cause ends of a Multicausation.
		 */

        ref :>> annotatedElement : SysML::Usage;
        ref :>> baseType = causes as SysML::Usage;
    }

    metadata def <effect> EffectMetadata :> SemanticMetadata {
        doc /*
		 * EffectMetadata identifies a usage as being an effect occurrence.
		 * It is intended to be used to tag the effect ends of a Multicausation.
		 */

        ref :>> annotatedElement : SysML::Usage;
        ref :>> baseType = effects as SysML::Usage;
    }

    metadata def CausationMetadata {
        doc /*
		 * CausationMetadata allows for the specification of additional metadata about
		 * a cause-effect connection definition or usage.
		 */

        ref :> annotatedElement : SysML::ConnectionDefinition;
        ref :> annotatedElement : SysML::ConnectionUsage;

        attribute isNecessary : Boolean default = false {
            doc /* 
			 * Whether all the causes are necessary for all the effects to occur.
			 * If this is false (the default), then some or all of the effects may 
			 * still have occurred even if some of the causes did not.
			 */
        }

        attribute isSufficient : Boolean default = false {
            doc /*
			 * Whether the causes were sufficient for all the effects to occur.
			 * If this is false (the default), then it may be the case that some
			 * other occurrences were also necessary for some or all of the effects
			 * to have occurred.
			 */
        }

        attribute probability : Real [0..1] {
            doc /* The probability that the causes will actually result in effects occurring. */
        }
    }

    metadata def <multicausation> MulticausationSemanticMetadata :> CausationMetadata, SemanticMetadata {
        doc /*
		 * MulticausationMetadata is SemanticMetadata for a Multicausation connection.
		 */

        ref :>> baseType = multicausations meta SysML::Usage;
    }

    metadata def <causation> CausationSemanticMetadadata :> CausationMetadata, SemanticMetadata {
        doc /*
		 * CausationMetadata is SemanticMetadata for a Causation connection.
		 */

        ref :>> baseType = causations meta SysML::Usage;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'CauseAndEffect'
      (documentation)
      (namespace_import public -> 'CausationConnections'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (membership_import private -> 'Metaobjects::SemanticMetadata'[unresolved])
      (metadata_def 'CauseMetadata' :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::Usage'[unresolved])
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'EffectMetadata' :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::Usage'[unresolved])
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'CausationMetadata'
        (documentation)
        (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::ConnectionDefinition'[unresolved])
        (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::ConnectionUsage'[unresolved])
        (attribute_usage composite 'isNecessary' : 'Boolean'[unresolved]
          (feature_value (default =))
          (documentation))
        (attribute_usage composite 'isSufficient' : 'Boolean'[unresolved]
          (feature_value (default =))
          (documentation))
        (attribute_usage composite 'probability' : 'Real'[unresolved]
          (multiplicity_range [0..1])
          (documentation)))
      (metadata_def 'MulticausationSemanticMetadata' :> 'CauseAndEffect::CausationMetadata'[metadata_def] :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'CausationSemanticMetadadata' :> 'CauseAndEffect::CausationMetadata'[metadata_def] :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=)))))))
~~~
