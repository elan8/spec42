# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/OccurrenceFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package OccurrenceFunctions {
	doc
	/*
	 * This package defines utility functions that operate on occurrences, primarily related to 
	 * time during which those occurrences exist.
	 */
	
	private import Occurrences::Occurrence;
	private import Occurrences::HappensDuring;
	private import ScalarValues::Boolean;
	private import ScalarValues::Positive;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::size;
	private import SequenceFunctions::add;
	private import SequenceFunctions::addAt;
	private import SequenceFunctions::remove;
	private import SequenceFunctions::removeAt;
	private import ControlFunctions::forAll;
	 
	function '==='  specializes BaseFunctions::'===' { 
		doc
		/*
		 * Test whether two occurrences are portions of the same life. That is, whether they 
		 * represent different portions of the same entity (colloquially, whether they have
		 * the same "identity").
		 */
		 
		in x: Occurrence[0..1]; 
		in y: Occurrence[0..1];
		
		return : Boolean[1] = x.portionOfLife == y.portionOfLife;
	}
	
	function isDuring {
		doc
		/*
		 * Test whether a performance of this function happens during the input occurrence.
		 */
		
		in occ: Occurrence[1];
		
		private connector all during: HappensDuring[0..1] from self to occ;
		
		return : Boolean[1] = notEmpty(during);
	}
	
	function create {
		doc
		/*
		 * Ensure that the start of a given occurrence happens during a performance of this
		 * function. The occurrence is also returned from the function.
		 */
		
		inout occ: Occurrence[1];
			
		private connector : HappensDuring from occ.startShot to self;	
		
		return : Occurrence[1] = occ;
	}
	
	function destroy {
		doc
		/*
		 * Ensure that the end of a given occurrence happens during a performance of this
		 * function. The occurrence is also returned from the function.
		 */

		inout occ: Occurrence[0..1];
		
		private connector : HappensDuring from [0..1] occ.endShot to self;
		
		return : Occurrence[0..1] = occ;
	}
	
	function addNew {
		doc
		/*
		 * Add a newly created occurrence to the given group of occurrences and return the
		 * new occurrence.
		 */

		inout group: Occurrence[0..*] nonunique;
		inout occ: Occurrence[1];
		
		private composite step : add {
			inout seq1 = group;
			in seq2 = create(occ);
		}
		
		return : Occurrence[1] = occ;
	}
	
	function addNewAt {
		doc
		/*
		 * Add a newly created occurrence to the given ordered group of occurrences at the given
		 * index and return the new occurrence.
		 */

		inout group: Occurrence[0..*] ordered nonunique;
		inout occ: Occurrence[1];
		in index: Positive[1];
		
		private composite step : addAt {
			inout seq = group;
			in values = create(occ);
			in startIndex = index;
		}
		
		return : Occurrence[1] = occ;
	}
	
	behavior removeOld {
		doc
		/*
		 * Remove a given occurrence from a group of occurrences and destroy it.
		 */

		inout group: Occurrence[0..*] nonunique;
		inout occ: Occurrence[0..1];
		
		private composite step removeStep : remove {
			inout seq = group;
			in values = occ;
		}
		private succession removeStep then destroyStep;
		private composite step destroyStep : destroy {
			inout occ = removeOld::occ;
		}
		
	}
	
	behavior removeOldAt {
		doc
		/*
		 * Removes the occurrence at a given index in an ordered group of occurrences 
		 * and destroy it.
		 */
		inout group: Occurrence[0..*] ordered nonunique;
		in index: Positive[1];
		
		private feature oldOcc = group#(index);
		
		private composite step removeStep : remove {
			inout seq = group;
			in index = removeOldAt::index;
		}
		private succession removeStep then destroyStep;
		private composite step destroyStep : destroy {
			inout occ = oldOcc;
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 40))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwConnector,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFrom,Ident,KwTo,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwConnector,Colon,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwConnector,Colon,Ident,KwFrom,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,KwTo,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwComposite,KwStep,Colon,Ident,OpenCurly,
KwInout,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwComposite,KwStep,Colon,Ident,OpenCurly,
KwInout,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,OpenCurly,
KwDoc,
RegularComment,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwComposite,KwStep,Ident,Colon,Ident,OpenCurly,
KwInout,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPrivate,KwSuccession,Ident,KwThen,Ident,Semicolon,
KwPrivate,KwComposite,KwStep,Ident,Colon,Ident,OpenCurly,
KwInout,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,OpenCurly,
KwDoc,
RegularComment,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwFeature,Ident,Eq,Ident,Hash,OpenParen,Ident,CloseParen,Semicolon,
KwPrivate,KwComposite,KwStep,Ident,Colon,Ident,OpenCurly,
KwInout,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPrivate,KwSuccession,Ident,KwThen,Ident,Semicolon,
KwPrivate,KwComposite,KwStep,Ident,Colon,Ident,OpenCurly,
KwInout,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'OccurrenceFunctions'
    (documentation)
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensDuring')
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::Positive')
    (import_decl private 'SequenceFunctions::notEmpty')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'SequenceFunctions::add')
    (import_decl private 'SequenceFunctions::addAt')
    (import_decl private 'SequenceFunctions::remove')
    (import_decl private 'SequenceFunctions::removeAt')
    (import_decl private 'ControlFunctions::forAll')
    (function_def
      (documentation)
      (feature_def in 'x' : 'Occurrence' multiplicity)
      (feature_def in 'y' : 'Occurrence' multiplicity)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'occ' : 'Occurrence' multiplicity)
      (connector_def private 'during' : 'HappensDuring' multiplicity
        (connector_end)
        (connector_end))
      (return_member))
    (function_def
      (documentation)
      (feature_def inout 'occ' : 'Occurrence' multiplicity)
      (connector_def private : 'HappensDuring'
        (connector_end)
        (connector_end))
      (return_member))
    (function_def
      (documentation)
      (feature_def inout 'occ' : 'Occurrence' multiplicity)
      (connector_def private : 'HappensDuring'
        (connector_end)
        (connector_end))
      (return_member))
    (function_def
      (documentation)
      (feature_def inout 'group' : 'Occurrence' multiplicity nonunique)
      (feature_def inout 'occ' : 'Occurrence' multiplicity)
      (step_def
        (feature_def inout 'seq1' value)
        (feature_def in 'seq2' value))
      (return_member))
    (function_def
      (documentation)
      (feature_def inout 'group' : 'Occurrence' multiplicity ordered nonunique)
      (feature_def inout 'occ' : 'Occurrence' multiplicity)
      (feature_def in 'index' : 'Positive' multiplicity)
      (step_def
        (feature_def inout 'seq' value)
        (feature_def in 'values' value)
        (feature_def in 'startIndex' value))
      (return_member))
    (behavior_def
      (documentation)
      (feature_def inout 'group' : 'Occurrence' multiplicity nonunique)
      (feature_def inout 'occ' : 'Occurrence' multiplicity)
      (step_def
        (feature_def inout 'seq' value)
        (feature_def in 'values' value))
      (succession_def private
        (connector_end)
        (connector_end))
      (step_def
        (feature_def inout 'occ' value)))
    (behavior_def
      (documentation)
      (feature_def inout 'group' : 'Occurrence' multiplicity ordered nonunique)
      (feature_def in 'index' : 'Positive' multiplicity)
      (feature_def private 'oldOcc' value)
      (step_def
        (feature_def inout 'seq' value)
        (feature_def in 'index' value))
      (succession_def private
        (connector_end)
        (connector_end))
      (step_def
        (feature_def inout 'occ' value)))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'BaseFunctions::==='
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'add'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'addAt'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'remove'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'remove'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BaseFunctions::==='
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'add'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'addAt'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'remove'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'remove'
~~~
# FORMAT
~~~sysml
standard library package OccurrenceFunctions {
	doc
	/*
	 * This package defines utility functions that operate on occurrences, primarily related to 
	 * time during which those occurrences exist.
	 */
	
	private import Occurrences::Occurrence;
	private import Occurrences::HappensDuring;
	private import ScalarValues::Boolean;
	private import ScalarValues::Positive;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::size;
	private import SequenceFunctions::add;
	private import SequenceFunctions::addAt;
	private import SequenceFunctions::remove;
	private import SequenceFunctions::removeAt;
	private import ControlFunctions::forAll;
	 
	function '==='  specializes BaseFunctions::'===' { 
		doc
		/*
		 * Test whether two occurrences are portions of the same life. That is, whether they 
		 * represent different portions of the same entity (colloquially, whether they have
		 * the same "identity").
		 */
		 
		in x: Occurrence[0..1]; 
		in y: Occurrence[0..1];
		
		return : Boolean[1] = x.portionOfLife == y.portionOfLife;
	}
	
	function isDuring {
		doc
		/*
		 * Test whether a performance of this function happens during the input occurrence.
		 */
		
		in occ: Occurrence[1];
		
		private connector all during: HappensDuring[0..1] from self to occ;
		
		return : Boolean[1] = notEmpty(during);
	}
	
	function create {
		doc
		/*
		 * Ensure that the start of a given occurrence happens during a performance of this
		 * function. The occurrence is also returned from the function.
		 */
		
		inout occ: Occurrence[1];
			
		private connector : HappensDuring from occ.startShot to self;	
		
		return : Occurrence[1] = occ;
	}
	
	function destroy {
		doc
		/*
		 * Ensure that the end of a given occurrence happens during a performance of this
		 * function. The occurrence is also returned from the function.
		 */

		inout occ: Occurrence[0..1];
		
		private connector : HappensDuring from [0..1] occ.endShot to self;
		
		return : Occurrence[0..1] = occ;
	}
	
	function addNew {
		doc
		/*
		 * Add a newly created occurrence to the given group of occurrences and return the
		 * new occurrence.
		 */

		inout group: Occurrence[0..*] nonunique;
		inout occ: Occurrence[1];
		
		private composite step : add {
			inout seq1 = group;
			in seq2 = create(occ);
		}
		
		return : Occurrence[1] = occ;
	}
	
	function addNewAt {
		doc
		/*
		 * Add a newly created occurrence to the given ordered group of occurrences at the given
		 * index and return the new occurrence.
		 */

		inout group: Occurrence[0..*] ordered nonunique;
		inout occ: Occurrence[1];
		in index: Positive[1];
		
		private composite step : addAt {
			inout seq = group;
			in values = create(occ);
			in startIndex = index;
		}
		
		return : Occurrence[1] = occ;
	}
	
	behavior removeOld {
		doc
		/*
		 * Remove a given occurrence from a group of occurrences and destroy it.
		 */

		inout group: Occurrence[0..*] nonunique;
		inout occ: Occurrence[0..1];
		
		private composite step removeStep : remove {
			inout seq = group;
			in values = occ;
		}
		private succession removeStep then destroyStep;
		private composite step destroyStep : destroy {
			inout occ = removeOld::occ;
		}
		
	}
	
	behavior removeOldAt {
		doc
		/*
		 * Removes the occurrence at a given index in an ordered group of occurrences 
		 * and destroy it.
		 */
		inout group: Occurrence[0..*] ordered nonunique;
		in index: Positive[1];
		
		private feature oldOcc = group#(index);
		
		private composite step removeStep : remove {
			inout seq = group;
			in index = removeOldAt::index;
		}
		private succession removeStep then destroyStep;
		private composite step destroyStep : destroy {
			inout occ = oldOcc;
		}
		
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6146f0929fa3d8912c95a049ca64ad61bc49931527dede863636981fc6ad8bb1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions"))) (kind "package") (name "OccurrenceFunctions") (declared-name "OccurrenceFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 3713))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 9) (character 1)) (end (line 9) (character 38))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 37))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (range (start (line 8) (character 1)) (end (line 8) (character 43))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 42))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 7) (character 1)) (end (line 7) (character 40))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 39))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (range (start (line 10) (character 1)) (end (line 10) (character 39))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 38))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 3713))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::add"))) (kind "import") (name "add") (declared-name "add") (range (start (line 13) (character 1)) (end (line 13) (character 39))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::add") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 38))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::addAt"))) (kind "import") (name "addAt") (declared-name "addAt") (range (start (line 14) (character 1)) (end (line 14) (character 41))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::addAt") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 40))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::addNew"))) (kind "kermlDecl") (name "addNew") (declared-name "addNew") (range (start (line 74) (character 1)) (end (line 74) (character 340))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::addNewAt"))) (kind "kermlDecl") (name "addNewAt") (declared-name "addNewAt") (range (start (line 92) (character 1)) (end (line 92) (character 431))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::create"))) (kind "kermlDecl") (name "create") (declared-name "create") (range (start (line 46) (character 1)) (end (line 46) (character 325))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::destroy"))) (kind "kermlDecl") (name "destroy") (declared-name "destroy") (range (start (line 60) (character 1)) (end (line 60) (character 331))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 17) (character 1)) (end (line 17) (character 41))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 40))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 19) (character 1)) (end (line 19) (character 393))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::isDuring"))) (kind "kermlDecl") (name "isDuring") (declared-name "isDuring") (range (start (line 33) (character 1)) (end (line 33) (character 272))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (range (start (line 11) (character 1)) (end (line 11) (character 44))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 43))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::remove"))) (kind "import") (name "remove") (declared-name "remove") (range (start (line 15) (character 1)) (end (line 15) (character 42))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::remove") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 41))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::removeAt"))) (kind "import") (name "removeAt") (declared-name "removeAt") (range (start (line 16) (character 1)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::removeAt") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 43))))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::removeOld"))) (kind "kermlDecl") (name "removeOld") (declared-name "removeOld") (range (start (line 112) (character 1)) (end (line 112) (character 424))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::removeOldAt"))) (kind "kermlDecl") (name "removeOldAt") (declared-name "removeOldAt") (range (start (line 132) (character 1)) (end (line 132) (character 505))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 12) (character 1)) (end (line 12) (character 40))) (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 9) (character 16)) (end (line 9) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (range (start (line 8) (character 16)) (end (line 8) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 7) (character 16)) (end (line 7) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (range (start (line 10) (character 16)) (end (line 10) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::add"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::add") (range (start (line 13) (character 16)) (end (line 13) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::addAt"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::addAt") (range (start (line 14) (character 16)) (end (line 14) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 17) (character 16)) (end (line 17) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (range (start (line 11) (character 16)) (end (line 11) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::remove"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::remove") (range (start (line 15) (character 16)) (end (line 15) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::removeAt"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::removeAt") (range (start (line 16) (character 16)) (end (line 16) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 12) (character 16)) (end (line 12) (character 39))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
