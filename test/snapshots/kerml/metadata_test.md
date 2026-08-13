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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 5 2) (end 5 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 2) (end 5 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 6 2) (end 6 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 2) (end 6 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 7 2) (end 7 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 2) (end 7 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 8 2) (end 8 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 2) (end 8 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 2) (end 13 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 2) (end 13 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 15 2) (end 15 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 18 1) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 1) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 24 1) (end 29 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 1) (end 29 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 31 1) (end 32 1))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 31 1) (end 32 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 34 1) (end 38 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 1) (end 38 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 40 4) (end 40 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 41 4) (end 43 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 41 4) (end 43 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 45 4) (end 49 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 4) (end 49 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 4) (end 51 7))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 51 7) (end 53 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 7) (end 53 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:25096972910a2fb98dff838639a8b58ba47e584512e0c981b5c96856f61fec70") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "User Defined Extensions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions"))) (kind library-package) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metadata_test.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions")))))
  )
)
~~~
