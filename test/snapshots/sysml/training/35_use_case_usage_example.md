# META
~~~ini
description=SysML Training 35 (Use Cases): Use Case Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Use Case Usage Example' {
	
	private import 'Use Case Definition Example'::*;
	
	part def 'Fuel Station';
	
	use case 'provide transportation' : 'Provide Transportation' {
	    subject vehicle;
	    	
		first start;
		
		then include use case 'enter vehicle' : 'Enter Vehicle' {
		    subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then use case 'drive vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor environment = 'provide transportation'::environment;
			
			include 'add fuel'[0..*] { 
                subject vehicle;
				actor fueler = driver;
			}
		}
		
		then include use case 'exit vehicle' : 'Exit Vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then done;		
	}
	
	use case 'add fuel' {
		subject vehicle : Vehicle;
		actor fueler : Person;
		actor 'fuel station' : 'Fuel Station';
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/35_use_case_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 37) (end 6 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 9 2) (end 9 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 2) (end 11 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 7) (end 11 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 15) (end 11 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 19) (end 11 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 24) (end 11 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 11 40) (end 15 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 17 2) (end 26 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 2) (end 28 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 7) (end 28 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 15) (end 28 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 19) (end 28 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 24) (end 28 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 28 39) (end 32 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 34 2) (end 34 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 20) (end 38 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 17) (end 39 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:978b14a368c7e7a64320e8cc6ffcfb059789f65349b6bb1baceb6c8cbaf78168") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (path (named (kind package) (name "Use Case Usage Example")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Use Case Definition Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::Fuel Station"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel Station"))))
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::fueler"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person"))))
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Provide Transportation")) (expressionOperand (reference "then")) (expressionOperand (reference "include")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "enter vehicle")) (expressionOperand (reference "then")) (expressionOperand (reference "include")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "exit vehicle"))))
    (declaration (id (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation::vehicle"))) (kind subject) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (path (named (kind package) (name "Use Case Usage Example")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Use Case Definition Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel Station")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::Fuel Station")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::fueler"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind featureTyping) (ordinal 0))
      (authored-target "Provide Transportation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 0))
      (authored-target "then")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 1))
      (authored-target "include")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 2))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 3))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 4))
      (authored-target "enter vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 5))
      (authored-target "then")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 6))
      (authored-target "include")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 7))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 8))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 9))
      (authored-target "exit vehicle")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (target (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::Fuel Station"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (state unresolved-operand))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 2 16) (end 2 48)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (path (named (kind package) (name "Use Case Usage Example")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Use Case Definition Example")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 40 25) (end 40 39)) (probe (position 40 25))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel Station")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::Fuel Station")))))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 39 17) (end 39 23)) (probe (position 39 17))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::fueler"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 38 20) (end 38 27)) (probe (position 38 20))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::add fuel::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 6 37) (end 6 61)) (probe (position 6 37))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind featureTyping) (ordinal 0) (authored-target "Provide Transportation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 11 2) (end 11 6)) (probe (position 11 2))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 0) (authored-target "then")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 11 7) (end 11 14)) (probe (position 11 7))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 1) (authored-target "include")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 11 15) (end 11 18)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 2) (authored-target "use")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 11 19) (end 11 23)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 3) (authored-target "case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 11 24) (end 11 39)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 4) (authored-target "enter vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 28 2) (end 28 6)) (probe (position 28 2))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 5) (authored-target "then")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 28 7) (end 28 14)) (probe (position 28 7))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 6) (authored-target "include")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 28 15) (end 28 18)) (probe (position 28 15))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 7) (authored-target "use")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 28 19) (end 28 23)) (probe (position 28 19))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 8) (authored-target "case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/35_use_case_usage_example.md") (range (start 28 24) (end 28 38)) (probe (position 28 24))
    (reference (id (source (node (document "memory://snapshot/35_use_case_usage_example.md") (qualified-name "Use Case Usage Example::provide transportation"))) (kind expressionOperand) (ordinal 9) (authored-target "exit vehicle")
      (outcome (status unresolved)))
  )
)
~~~
