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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CauseAndEffect"))) (name "CauseAndEffect") (declared-name "CauseAndEffect")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "CauseAndEffect::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CauseAndEffect::*#import"))) (name "*") (declared-name "*"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (name "CausationMetadata") (declared-name "CausationMetadata")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (name "isNecessary") (declared-name "isNecessary") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (name "isSufficient") (declared-name "isSufficient") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (name "probability") (declared-name "probability") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::probability::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata")))))
              )
            )
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (name "CausationSemanticMetadadata") (declared-name "CausationSemanticMetadadata")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))) (name "CauseMetadata") (declared-name "CauseMetadata")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))) (name "EffectMetadata") (declared-name "EffectMetadata")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (name "MulticausationSemanticMetadata") (declared-name "MulticausationSemanticMetadata")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata"))) (name "SemanticMetadata") (declared-name "SemanticMetadata"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "CauseAndEffect::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::probability::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::probability"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::_documentation"))) (to (node (document "d0") (qualified-name "CauseAndEffect"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute"))) (to (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
