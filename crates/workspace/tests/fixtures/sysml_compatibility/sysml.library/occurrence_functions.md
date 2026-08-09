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
# FORMAT
~~~sysml
standard library package OccurrenceFunctions {
    doc /*
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
        doc /*
		 * Remove a given occurrence from a group of occurrences and destroy it.
		 */

        inout group: Occurrence [0..*] nonunique;
        inout occ: Occurrence [0..1];

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
        doc /*
		 * Removes the occurrence at a given index in an ordered group of occurrences 
		 * and destroy it.
		 */
        inout group: Occurrence [0..*] ordered nonunique;
        in index: Positive [1];

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
(model
  (namespace
    (library_package 'OccurrenceFunctions'
      (documentation)
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensDuring'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarValues::Positive'[unresolved])
      (membership_import private -> 'SequenceFunctions::notEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (membership_import private -> 'SequenceFunctions::add'[unresolved])
      (membership_import private -> 'SequenceFunctions::addAt'[unresolved])
      (membership_import private -> 'SequenceFunctions::remove'[unresolved])
      (membership_import private -> 'SequenceFunctions::removeAt'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (function_def '===' :> 'BaseFunctions::==='[unresolved]
        (documentation)
        (feature_def in 'x' : 'Occurrence'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Occurrence'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'isDuring'
        (documentation)
        (feature_def in 'occ' : 'Occurrence'[unresolved]
          (multiplicity_range [1]))
        (connector_def 'during' : 'HappensDuring'[unresolved]
          (multiplicity_range [0..1])
          (connector_end 'self')
          (connector_end 'occ'))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'create'
        (documentation)
        (feature_def inout 'occ' : 'Occurrence'[unresolved]
          (multiplicity_range [1]))
        (connector_def : 'HappensDuring'[unresolved]
          (connector_end 'occ.startShot')
          (connector_end 'self'))
        (return_parameter_membership
          (feature_def out : 'Occurrence'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'destroy'
        (documentation)
        (feature_def inout 'occ' : 'Occurrence'[unresolved]
          (multiplicity_range [0..1]))
        (connector_def : 'HappensDuring'[unresolved]
          (connector_end 'occ.endShot')
          (connector_end 'self'))
        (return_parameter_membership
          (feature_def out : 'Occurrence'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=)))))
      (function_def 'addNew'
        (documentation)
        (feature_def inout 'group' : 'Occurrence'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def inout 'occ' : 'Occurrence'[unresolved]
          (multiplicity_range [1]))
        (step_def composite : 'add'[unresolved]
          (feature_def inout 'seq1'
            (feature_value (=)))
          (feature_def in 'seq2'
            (feature_value (=))))
        (return_parameter_membership
          (feature_def out : 'Occurrence'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'addNewAt'
        (documentation)
        (feature_def inout ordered 'group' : 'Occurrence'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def inout 'occ' : 'Occurrence'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'index' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (step_def composite : 'addAt'[unresolved]
          (feature_def inout 'seq'
            (feature_value (=)))
          (feature_def in 'values'
            (feature_value (=)))
          (feature_def in 'startIndex'
            (feature_value (=))))
        (return_parameter_membership
          (feature_def out : 'Occurrence'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (behavior_def 'removeOld'
        (documentation)
        (feature_def inout 'group' : 'Occurrence'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def inout 'occ' : 'Occurrence'[unresolved]
          (multiplicity_range [0..1]))
        (step_def composite 'removeStep' : 'remove'[unresolved]
          (feature_def inout 'seq'
            (feature_value (=)))
          (feature_def in 'values'
            (feature_value (=))))
        (succession_def
          (connector_end 'removeStep')
          (connector_end 'destroyStep'))
        (step_def composite 'destroyStep' : 'OccurrenceFunctions::destroy'[function_def]
          (feature_def inout 'occ'
            (feature_value (=)))))
      (behavior_def 'removeOldAt'
        (documentation)
        (feature_def inout ordered 'group' : 'Occurrence'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'index' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (feature_def 'oldOcc'
          (feature_value (=)))
        (step_def composite 'removeStep' : 'remove'[unresolved]
          (feature_def inout 'seq'
            (feature_value (=)))
          (feature_def in 'index'
            (feature_value (=))))
        (succession_def
          (connector_end 'removeStep')
          (connector_end 'destroyStep'))
        (step_def composite 'destroyStep' : 'OccurrenceFunctions::destroy'[function_def]
          (feature_def inout 'occ'
            (feature_value (=))))))))
~~~
