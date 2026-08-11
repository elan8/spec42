# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_19-Materials with Properties
type=file
~~~
# SOURCE
~~~sysml
package '15_19-Materials with Properties' {
	private import ScalarValues::Real;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import SI::*;
	
    attribute def AtomicMassValue :> MassValue;
    
	attribute def TensileStrengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }		
	}
    
    attribute def TensileStrengthValue :> ScalarQuantityValue {
		attribute :>> num: Real;
    	attribute :>> mRef: TensileStrengthUnit;
    }
    
    attribute <'N/mm²'> 'newton per square millimetre' : TensileStrengthUnit = N / mm^2;

    part def Substance;
    part def Material :> Substance;

	/*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */

    part def Metal :> Material {
        attribute atomicMass: AtomicMassValue[1];
    }

    attribute def MaterialFraction {
        ref material: Material[1]; 
        attribute massFraction: MassFractionValue[1];
    }

    attribute def MassFractionValue :> DimensionOneValue;    

    part def Alloy :> Material {
        attribute fractions: MaterialFraction[2..*];
    }

    individual def Iron :> Metal {
        attribute :>> atomicMass = 55.845 [Da];
    }

    individual def Carbon :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 12.011[Da];
    }

    individual def Manganese :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 54.938[Da];
    }

    individual def Steel_980 :> Alloy {
	 	/*
		 * Particular example of high tensile strength steel.
		 */
 	
        attribute fraction1 :> fractions {
        	ref :>> material : Iron;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute fraction2 :> fractions {
        	ref :>> material : Carbon;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute fraction3 :> fractions {
        	ref :>> material : Manganese;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute tensileStrength: TensileStrengthValue = 980['N/mm²'];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_19_materials_with_properties.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 4) (end 6 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 1) (end 8 470))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 8) (end 9 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 8) (end 10 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 8) (end 11 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 4) (end 15 142))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 4) (end 41 57))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 79 8) (end 79 72))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,OpenCurly,
RegularComment,
KwAttribute,Ident,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_19-Materials with Properties''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'SI::*')
    (attribute_def 'AtomicMassValue' :> 'MassValue')
    (attribute_def 'TensileStrengthUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'TensileStrengthValue' :> 'ScalarQuantityValue'
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TensileStrengthUnit'))
    (attribute_usage ''newton per square millimetre'' : 'TensileStrengthUnit' value)
    (part_def 'Substance')
    (part_def 'Material' :> 'Substance')
    (comment)
    (part_def 'Metal' :> 'Material'
      (attribute_usage 'atomicMass' : 'AtomicMassValue' multiplicity))
    (attribute_def 'MaterialFraction'
      (ref_usage ref 'material' : 'Material' multiplicity)
      (attribute_usage 'massFraction' : 'MassFractionValue' multiplicity))
    (attribute_def 'MassFractionValue' :> 'DimensionOneValue')
    (part_def 'Alloy' :> 'Material'
      (attribute_usage 'fractions' : 'MaterialFraction' multiplicity))
    (individual_def individual 'Iron' :> 'Metal'
      (attribute_usage :>> 'atomicMass' value))
    (individual_def individual 'Carbon' :> 'Metal'
      (attribute_usage 'atomicMass' :>> 'Metal::atomicMass' value))
    (individual_def individual 'Manganese' :> 'Metal'
      (attribute_usage 'atomicMass' :>> 'Metal::atomicMass' value))
    (individual_def individual 'Steel_980' :> 'Alloy'
      (comment)
      (attribute_usage 'fraction1' :> 'fractions'
        (ref_usage ref :>> 'material' : 'Iron')
        (attribute_usage :>> 'massFraction' value))
      (attribute_usage 'fraction2' :> 'fractions'
        (ref_usage ref :>> 'material' : 'Carbon')
        (attribute_usage :>> 'massFraction' value))
      (attribute_usage 'fraction3' :> 'fractions'
        (ref_usage ref :>> 'material' : 'Manganese')
        (attribute_usage :>> 'massFraction' value))
      (attribute_usage 'tensileStrength' : 'TensileStrengthValue' value))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'DimensionOneValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'DimensionOneValue'
~~~
# FORMAT
~~~sysml
package '15_19-Materials with Properties' {
    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import SI::*;

    attribute def AtomicMassValue :> MassValue;

    attribute def TensileStrengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def TensileStrengthValue :> ScalarQuantityValue {
        attribute :>> num: Real;
        attribute :>> mRef: TensileStrengthUnit;
    }

    attribute <'N/mm²'> 'newton per square millimetre' : TensileStrengthUnit = N / mm^2;

    part def Substance;
    part def Material :> Substance;

    /*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */

    part def Metal :> Material {
        attribute atomicMass: AtomicMassValue[1];
    }

    attribute def MaterialFraction {
        ref material: Material[1];
        attribute massFraction: MassFractionValue[1];
    }

    attribute def MassFractionValue :> DimensionOneValue;

    part def Alloy :> Material {
        attribute fractions: MaterialFraction[2..*];
    }

    individual def Iron :> Metal {
        attribute :>> atomicMass = 55.845 [Da];
    }

    individual def Carbon :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 12.011[Da];
    }

    individual def Manganese :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 54.938[Da];
    }

    individual def Steel_980 :> Alloy {
        /*
		 * Particular example of high tensile strength steel.
		 */

        attribute fraction1 :> fractions {
            ref :>> material : Iron;
            attribute :>> massFraction = 0.9862[one];
        }

        attribute fraction2 :> fractions {
            ref :>> material : Carbon;
            attribute :>> massFraction = 0.9862[one];
        }

        attribute fraction3 :> fractions {
            ref :>> material : Manganese;
            attribute :>> massFraction = 0.9862[one];
        }

        attribute tensileStrength: TensileStrengthValue = 980['N/mm²'];
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "936a0f106bddd3d4ba27c475dffcd73db78b19519dc85723eee2aacedaa6fed4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (kind "package") (name "15_19-Materials with Properties") (declared-name "15_19-Materials with Properties") (range (start (line 0) (character 0)) (end (line 0) (character 2655))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 26))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 41))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 37))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 22))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 18))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind "part def") (name "Alloy") (declared-name "Alloy") (range (start (line 43) (character 4)) (end (line 43) (character 91))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Material") (range (start (line 43) (character 22)) (end (line 43) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind "attribute") (name "fractions") (declared-name "fractions") (range (start (line 44) (character 8)) (end (line 44) (character 52))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (authored (membership (kind Feature)) (relationships (typing (reference "MaterialFraction") (range none)) (typing (reference "MaterialFraction") (range (start (line 44) (character 29)) (end (line 44) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (kind "attribute def") (name "AtomicMassValue") (declared-name "AtomicMassValue") (range (start (line 6) (character 4)) (end (line 6) (character 47))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (kind "individual def") (name "Carbon") (declared-name "Carbon") (range (start (line 51) (character 4)) (end (line 51) (character 107))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Metal") (range (start (line 51) (character 29)) (end (line 51) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (range (start (line 52) (character 8)) (end (line 52) (character 64))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Metal::atomicMass") (range (start (line 52) (character 33)) (end (line 52) (character 50)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (kind "individual def") (name "Iron") (declared-name "Iron") (range (start (line 47) (character 4)) (end (line 47) (character 88))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Metal") (range (start (line 47) (character 27)) (end (line 47) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (range (start (line 48) (character 8)) (end (line 48) (character 47))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "atomicMass") (range (start (line 48) (character 22)) (end (line 48) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (kind "individual def") (name "Manganese") (declared-name "Manganese") (range (start (line 55) (character 4)) (end (line 55) (character 110))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Metal") (range (start (line 55) (character 32)) (end (line 55) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (range (start (line 56) (character 8)) (end (line 56) (character 64))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Metal::atomicMass") (range (start (line 56) (character 33)) (end (line 56) (character 50)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (kind "attribute def") (name "MassFractionValue") (declared-name "MassFractionValue") (range (start (line 41) (character 4)) (end (line 41) (character 57))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (kind "part def") (name "Material") (declared-name "Material") (range (start (line 23) (character 4)) (end (line 23) (character 35))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Substance") (range (start (line 23) (character 25)) (end (line 23) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (kind "attribute def") (name "MaterialFraction") (declared-name "MaterialFraction") (range (start (line 36) (character 4)) (end (line 36) (character 132))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind "attribute") (name "massFraction") (declared-name "massFraction") (range (start (line 38) (character 8)) (end (line 38) (character 53))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassFractionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind "ref") (name "material") (declared-name "material") (range (start (line 37) (character 8)) (end (line 37) (character 34))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Material") (range (start (line 37) (character 21)) (end (line 37) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (kind "part def") (name "Metal") (declared-name "Metal") (range (start (line 32) (character 4)) (end (line 32) (character 88))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Material") (range (start (line 32) (character 22)) (end (line 32) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (range (start (line 33) (character 8)) (end (line 33) (character 49))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (authored (membership (kind Feature)) (relationships (typing (reference "AtomicMassValue") (range none)) (typing (reference "AtomicMassValue") (range (start (line 33) (character 30)) (end (line 33) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (kind "individual def") (name "Steel_980") (declared-name "Steel_980") (range (start (line 59) (character 4)) (end (line 59) (character 637))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Alloy") (range (start (line 59) (character 32)) (end (line 59) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (kind "attribute") (name "fraction1") (declared-name "fraction1") (range (start (line 64) (character 8)) (end (line 64) (character 137))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "fractions") (range (start (line 64) (character 31)) (end (line 64) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (kind "attribute") (name "fraction2") (declared-name "fraction2") (range (start (line 69) (character 8)) (end (line 69) (character 139))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "fractions") (range (start (line 69) (character 31)) (end (line 69) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (kind "attribute") (name "fraction3") (declared-name "fraction3") (range (start (line 74) (character 8)) (end (line 74) (character 142))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "fractions") (range (start (line 74) (character 31)) (end (line 74) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (kind "attribute") (name "tensileStrength") (declared-name "tensileStrength") (range (start (line 79) (character 8)) (end (line 79) (character 72))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (typing (reference "TensileStrengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Substance"))) (kind "part def") (name "Substance") (declared-name "Substance") (range (start (line 22) (character 4)) (end (line 22) (character 23))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (kind "attribute def") (name "TensileStrengthUnit") (declared-name "TensileStrengthUnit") (range (start (line 8) (character 1)) (end (line 8) (character 470))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 11) (character 8)) (end (line 11) (character 105))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 9) (character 8)) (end (line 9) (character 103))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 10) (character 8)) (end (line 10) (character 100))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 12) (character 8)) (end (line 12) (character 102))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 12) (character 22)) (end (line 12) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (kind "attribute def") (name "TensileStrengthValue") (declared-name "TensileStrengthValue") (range (start (line 15) (character 4)) (end (line 15) (character 142))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 17) (character 5)) (end (line 17) (character 45))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TensileStrengthUnit") (range none)) (redefinition (reference "mRef") (range (start (line 17) (character 19)) (end (line 17) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 16) (character 2)) (end (line 16) (character 26))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 16) (character 16)) (end (line 16) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind "attribute def") (name "newton per square millimetre") (declared-name "newton per square millimetre") (range (start (line 20) (character 4)) (end (line 20) (character 89))) (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensileStrengthUnit") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 2) (character 16)) (end (line 2) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 3) (character 16)) (end (line 3) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 4) (character 16)) (end (line 4) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind specialization) (ordinal 0)) (authored-target "Material") (range (start (line 43) (character 22)) (end (line 43) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0)) (authored-target "MaterialFraction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 1)) (authored-target "MaterialFraction") (range (start (line 44) (character 29)) (end (line 44) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (kind specialization) (ordinal 0)) (authored-target "Metal") (range (start (line 51) (character 29)) (end (line 51) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "Metal::atomicMass") (range (start (line 52) (character 33)) (end (line 52) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (kind specialization) (ordinal 0)) (authored-target "Metal") (range (start (line 47) (character 27)) (end (line 47) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "atomicMass") (range (start (line 48) (character 22)) (end (line 48) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (kind specialization) (ordinal 0)) (authored-target "Metal") (range (start (line 55) (character 32)) (end (line 55) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "Metal::atomicMass") (range (start (line 56) (character 33)) (end (line 56) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (kind specialization) (ordinal 0)) (authored-target "Substance") (range (start (line 23) (character 25)) (end (line 23) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Substance")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFractionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0)) (authored-target "Material") (range (start (line 37) (character 21)) (end (line 37) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (kind specialization) (ordinal 0)) (authored-target "Material") (range (start (line 32) (character 22)) (end (line 32) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0)) (authored-target "AtomicMassValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 1)) (authored-target "AtomicMassValue") (range (start (line 33) (character 30)) (end (line 33) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (kind specialization) (ordinal 0)) (authored-target "Alloy") (range (start (line 59) (character 32)) (end (line 59) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (kind subsetting) (ordinal 0)) (authored-target "fractions") (range (start (line 64) (character 31)) (end (line 64) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (kind subsetting) (ordinal 0)) (authored-target "fractions") (range (start (line 69) (character 31)) (end (line 69) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (kind subsetting) (ordinal 0)) (authored-target "fractions") (range (start (line 74) (character 31)) (end (line 74) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 12) (character 22)) (end (line 12) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 17) (character 19)) (end (line 17) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 16) (character 16)) (end (line 16) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Substance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
