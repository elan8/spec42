# META
~~~ini
description=KerML Simple Tests: MetadataTest
type=file
~~~
# SOURCE
~~~kerml
package MetadataTest {
	private import 'User Defined Extensions'::*;
	
	library package 'User Defined Extensions' {
		
		datatype ClassificationLevel :> ScalarValues::Natural;
		feature uncl[1] : ClassificationLevel = 0;
		feature conf[1] : ClassificationLevel = 1;
		feature secret[1] : ClassificationLevel = 2;
		
		metaclass Classified {
			feature :>> annotatedElement : KerML::Feature;
			feature classificationLevel : ClassificationLevel;
		}
		
		metaclass Security;
	}
	
	feature x {
		metadata Classified {
			classificationLevel = conf;
		}
	}
	
	feature y {
		@Classified {
			classificationLevel = conf;
		}
		@Security;
	}
	
	private #Classified #Security feature z1;
	abstract #Classified z2;
	
	feature z {
	    #Security #Classified metadata Classified {
	        classificationLevel = secret;
	    }
	}
	
    class CC;
    struct SS {
        feature cc : CC;
    }
    
    metaclass M :> Metaobjects::SemanticMetadata {
      :>> annotatedElement : KerML::Class;
      :>> baseType = if annotatedElement istype KerML::Structure ? 
                         SS meta KerML::Type else CC meta KerML::Class;
    }
    
    #M struct T {
        feature :>> cc;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/metadata_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 34) (end 5 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 15) (end 11 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 34) (end 11 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 15 2) (end 15 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 2) (end 19 10))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 19 22) (end 22 1))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 19 22) (end 22 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 3) (end 28 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 35 5) (end 38 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 45 19) (end 45 48))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 46 6) (end 47 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 10) (end 47 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 24) (end 47 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 48) (end 47 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 48 33) (end 48 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 48 58) (end 48 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 4) (end 51 7))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 20) (end 52 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:25096972910a2fb98dff838639a8b58ba47e584512e0c981b5c96856f61fec70") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "User Defined Extensions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::CC"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::M"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobjects::SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")) (expressionOperand (reference "annotatedElement")) (expressionOperand (reference "SS")) (expressionOperand (reference "CC")) (typeCheckTarget (reference "KerML::Structure")) (metaCastTarget (reference "KerML::Type")) (metaCastTarget (reference "KerML::Class"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS::cc"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CC"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::T"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "T")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cc"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarValues::Natural"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind kerml-classifier) (name "Classified")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KerML::Feature")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassificationLevel"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassificationLevel"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::secret"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassificationLevel"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::uncl"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassificationLevel"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "metadata")) (expressionOperand (reference "Classified"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Classified")) (metadataAnnotation (reference "Security"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-feature) (name "y")) (anonymous (kind metadata) (ordinal 0)))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-feature) (name "y")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "classificationLevel")))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "conf"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::M"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind expressionOperand) (ordinal 1))
      (authored-target "SS")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind expressionOperand) (ordinal 2))
      (authored-target "CC")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::CC")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "KerML::Structure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind metaCastTarget) (ordinal 0))
      (authored-target "KerML::Type")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind metaCastTarget) (ordinal 1))
      (authored-target "KerML::Class")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS::cc"))) (kind featureTyping) (ordinal 0))
      (authored-target "CC")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::CC")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "T")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "cc")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind kerml-classifier) (name "Classified")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind featureTyping) (ordinal 0))
      (authored-target "KerML::Feature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind kerml-classifier) (name "Classified")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::secret"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::uncl"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (kind expressionOperand) (ordinal 0))
      (authored-target "metadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (kind expressionOperand) (ordinal 1))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 1))
      (authored-target "Security")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-feature) (name "y")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "classificationLevel")))))) (kind expressionOperand) (ordinal 0))
      (authored-target "conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::CC"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS::cc"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::CC"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS::cc"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::secret"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::secret"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::uncl"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::uncl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-feature) (name "y")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "classificationLevel")))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-feature) (name "y")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "classificationLevel")))))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf"))) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::secret"))) (value (kind integer) (integer 2)))
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::uncl"))) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-feature) (name "y")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "classificationLevel")))))) (value (kind integer) (integer 1)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metadata_test.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 45 19) (end 45 48)) (probe (position 45 19))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::M"))) (kind specialization) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 47 10) (end 47 18)) (probe (position 47 10))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 47 24) (end 47 40)) (probe (position 47 24))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 48 25) (end 48 27)) (probe (position 48 25))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind expressionOperand) (ordinal 1) (authored-target "SS")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 48 50) (end 48 52)) (probe (position 48 50))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind expressionOperand) (ordinal 2) (authored-target "CC")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::CC")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 47 48) (end 47 64)) (probe (position 47 48))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind typeCheckTarget) (ordinal 0) (authored-target "KerML::Structure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 48 33) (end 48 44)) (probe (position 48 33))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind metaCastTarget) (ordinal 0) (authored-target "KerML::Type")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 48 58) (end 48 70)) (probe (position 48 58))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "M")) (anonymous (kind default-reference) (ordinal 0)))))) (kind metaCastTarget) (ordinal 1) (authored-target "KerML::Class")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 42 21) (end 42 23)) (probe (position 42 21))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::SS::cc"))) (kind featureTyping) (ordinal 0) (authored-target "CC")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::CC")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 52 20) (end 52 22)) (probe (position 52 20))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-classifier) (name "T")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "cc")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 5 34) (end 5 55)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind specialization) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 11 34) (end 11 48)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind kerml-classifier) (name "Classified")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind featureTyping) (ordinal 0) (authored-target "KerML::Feature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 11 15) (end 11 31)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind kerml-classifier) (name "Classified")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 12 33) (end 12 52)) (probe (position 12 33))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0) (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 7 20) (end 7 39)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf"))) (kind featureTyping) (ordinal 0) (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 8 22) (end 8 41)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::secret"))) (kind featureTyping) (ordinal 0) (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 6 20) (end 6 39)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::uncl"))) (kind featureTyping) (ordinal 0) (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 19 2) (end 19 10)) (probe (position 19 2))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (kind expressionOperand) (ordinal 0) (authored-target "metadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 19 11) (end 19 21)) (probe (position 19 11))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (kind expressionOperand) (ordinal 1) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 25 3) (end 25 13)) (probe (position 25 3))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 28 3) (end 28 11)) (probe (position 28 3))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 1) (authored-target "Security")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 26 25) (end 26 29)) (probe (position 26 25))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind kerml-feature) (name "y")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "classificationLevel")))))) (kind expressionOperand) (ordinal 0) (authored-target "conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::conf")))))
  )
)
~~~
