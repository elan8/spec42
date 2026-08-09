# META
~~~ini
description=KerML KerML Spec Annex A: A-3-6-Sequences
type=file
~~~
# SOURCE
~~~kerml

package SequencesModelToBeExecuted {
	doc
	/* 
	 */

	behavior Manufacture {
		step paint : Paint [1];
		step dry : Dry [*];
		succession p_before_d first [1] paint then [1] dry;
		step ship : Ship [*];
		succession d_before_s first [1] dry then [1] ship;
	}
	behavior Paint;
	behavior Dry;
	behavior Ship;
}

package SequencesExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import SequencesModelToBeExecuted::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;

	#atom
	behavior MyPaint specializes Paint;
	#atom
	behavior MyDry specializes Dry;

	#atom
	assoc MyPaint_Before_Dry_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyPaint;
		end feature redefines laterOccurrence : MyDry;
	}

	behavior MyManufactureStepsPD unions MyPaint, MyDry;

	#atom
	behavior MyShip specializes Ship;

	#atom
	assoc MyDry_Before_Ship_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyDry;
		end feature redefines laterOccurrence : MyShip;
	}

	behavior MyManufactureStepsPDS unions MyManufactureStepsPD, MyShip;

	#atom
	behavior MyManufacture specializes Manufacture {
		feature redefines timeEnclosedOccurrences : MyManufactureStepsPDS [3];
		step redefines paint : MyPaint;
		step redefines dry : MyDry [1];
		succession redefines p_before_d : MyPaint_Before_Dry_Link [1] first paint then dry;
		step redefines ship : MyShip [1];
		succession redefines d_before_s : MyDry_Before_Ship_Link [1] first dry then ship;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwBehavior,Ident,OpenCurly,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,Semicolon,
KwBehavior,Ident,Semicolon,
KwBehavior,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'SequencesModelToBeExecuted'
    (documentation)
    (behavior_def
      (step_def)
      (step_def)
      (succession_def 'p_before_d'
        (connector_end)
        (connector_end))
      (step_def)
      (succession_def 'd_before_s'
        (connector_end)
        (connector_end)))
    (behavior_def)
    (behavior_def)
    (behavior_def))
  (package_def 'SequencesExecution'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'SequencesModelToBeExecuted::*')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensBefore')
    (behavior_def)
    (behavior_def)
    (association_def #'atom' 'MyPaint_Before_Dry_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyPaint')
      (feature_def end :>> 'laterOccurrence' : 'MyDry'))
    (behavior_def)
    (behavior_def)
    (association_def #'atom' 'MyDry_Before_Ship_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyDry')
      (feature_def end :>> 'laterOccurrence' : 'MyShip'))
    (behavior_def)
    (behavior_def
      (feature_def :>> 'timeEnclosedOccurrences' : 'MyManufactureStepsPDS' multiplicity)
      (step_def)
      (step_def)
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end))
      (step_def)
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package SequencesModelToBeExecuted {
    doc /* 
	 */

    behavior Manufacture {
        step paint : Paint [1];
        step dry : Dry [*];
        succession p_before_d first [1] paint then [1] dry;
        step ship : Ship [*];
        succession d_before_s first [1] dry then [1] ship;
    }
    behavior Paint;
    behavior Dry;
    behavior Ship;
}

package SequencesExecution {
    doc /* 
	 */

    private import Atoms::*;
    private import SequencesModelToBeExecuted::*;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensBefore;

    #atom behavior MyPaint specializes Paint;
    #atom behavior MyDry specializes Dry;

    #atom assoc MyPaint_Before_Dry_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : MyPaint;
        end feature redefines laterOccurrence : MyDry;
    }

    behavior MyManufactureStepsPD unions MyPaint, MyDry;

    #atom behavior MyShip specializes Ship;

    #atom assoc MyDry_Before_Ship_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : MyDry;
        end feature redefines laterOccurrence : MyShip;
    }

    behavior MyManufactureStepsPDS unions MyManufactureStepsPD, MyShip;

    #atom behavior MyManufacture specializes Manufacture {
        feature redefines timeEnclosedOccurrences : MyManufactureStepsPDS [3];
        step redefines paint : MyPaint;
        step redefines dry : MyDry [1];
        succession redefines p_before_d : MyPaint_Before_Dry_Link [1]
        first paint then dry;
        step redefines ship : MyShip [1];
        succession redefines d_before_s : MyDry_Before_Ship_Link [1]
        first dry then ship;
    }
}
~~~
# EXPECTED
~~~
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'timeEnclosedOccurrences'
~~~
# PROBLEMS
~~~
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'timeEnclosedOccurrences'
~~~
# SMG
~~~
(model
  (namespace
    (package 'SequencesModelToBeExecuted'
      (documentation)
      (behavior_def 'Manufacture'
        (step_def 'paint' : 'SequencesModelToBeExecuted::Paint'[behavior_def]
          (multiplicity_range [1]))
        (step_def 'dry' : 'SequencesModelToBeExecuted::Dry'[behavior_def]
          (multiplicity_range [*]))
        (succession_def 'p_before_d'
          (connector_end 'paint')
          (connector_end 'dry'))
        (step_def 'ship' : 'SequencesModelToBeExecuted::Ship'[behavior_def]
          (multiplicity_range [*]))
        (succession_def 'd_before_s'
          (connector_end 'dry')
          (connector_end 'ship')))
      (behavior_def 'Paint')
      (behavior_def 'Dry')
      (behavior_def 'Ship'))
    (package 'SequencesExecution'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import private -> 'SequencesModelToBeExecuted'[package])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensBefore'[unresolved])
      (behavior_def 'MyPaint' :> 'SequencesModelToBeExecuted::Paint'[behavior_def])
      (behavior_def 'MyDry' :> 'SequencesModelToBeExecuted::Dry'[behavior_def])
      (association_def 'MyPaint_Before_Dry_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'SequencesExecution::MyPaint'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'SequencesExecution::MyDry'[behavior_def]))
      (behavior_def 'MyManufactureStepsPD'
        (unioning)
        (unioning))
      (behavior_def 'MyShip' :> 'SequencesModelToBeExecuted::Ship'[behavior_def])
      (association_def 'MyDry_Before_Ship_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'SequencesExecution::MyDry'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'SequencesExecution::MyShip'[behavior_def]))
      (behavior_def 'MyManufactureStepsPDS'
        (unioning)
        (unioning))
      (behavior_def 'MyManufacture' :> 'SequencesModelToBeExecuted::Manufacture'[behavior_def]
        (feature_def :>> 'timeEnclosedOccurrences'[unresolved] : 'SequencesExecution::MyManufactureStepsPDS'[behavior_def]
          (multiplicity_range [3]))
        (step_def :>> 'SequencesModelToBeExecuted::Manufacture::paint'[step_def] : 'SequencesExecution::MyPaint'[behavior_def])
        (step_def :>> 'SequencesModelToBeExecuted::Manufacture::dry'[step_def] : 'SequencesExecution::MyDry'[behavior_def]
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'paint')
          (connector_end 'dry'))
        (step_def :>> 'SequencesModelToBeExecuted::Manufacture::ship'[step_def] : 'SequencesExecution::MyShip'[behavior_def]
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'dry')
          (connector_end 'ship'))))))
~~~
