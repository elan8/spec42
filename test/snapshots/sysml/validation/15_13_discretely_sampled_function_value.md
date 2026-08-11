# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_13-Discretely Sampled Function Value
type=file
~~~
# SOURCE
~~~sysml
package '15_13-Discretely Sampled Function Value' {
	private import SampledFunctions::SampledFunction;
	private import SampledFunctions::SamplePair;
	private import Collections::Array;
	private import ISQ::*;
	private import SI::*;
	private import MeasurementReferences::*;
	private import Time::*;

	attribute def MissionElapsedTimeScale :> TimeScale {
		:>> unit = s;
		attribute :>> definitionalEpoch {
			:>> num = 0;
			:>> definition = "time instant zero at launch";
		}
		attribute definitionalEpochInUTC : Iso8601DateTime;
		
		// Map the definitional epoch (t = 0) of this scale to a reference epoch expressed in UTC
		// This modeled as a 1D coordinate transformation (translation only)
		attribute :>> transformation : CoordinateFramePlacement {
			:>> source = UTC;
			:>> origin = definitionalEpochInUTC;
			:>> basisDirections = 1 [UTC];
		}
  }

	attribute mets: MissionElapsedTimeScale { 
		doc
		/*
		 * Define mission elapsed time scale starting at given UTC date time (in microsecond resolution)
		 */
		:>> definitionalEpochInUTC { :>> val = "2020-08-23T22:42:32.924534Z";}		
	}

	attribute def MissionElapsedTimeValue :> TimeInstantValue {
		doc
		/*
		 * Define scalar quantity value type for mission elapsed time
		 */
	 	:>> mRef = mets; 
	}

	attribute spatialCF: CartesianSpatial3dCoordinateFrame[1] {
		doc
		/*
		 * Define Cartesian 3D coordinate systems for position and velocity
		 * Create a velocity coordinate system from the spatial coordinate system through division by second
		 */
	   :>> mRefs = (m, m, m);
	}
	attribute velocityCF: CartesianVelocity3dCoordinateFrame[1] = spatialCF/s;

	attribute def PositionAndVelocity {
		attribute position : CartesianPosition3dVector[1];
		attribute velocity : CartesianVelocity3dVector[1];
	}

	attribute def AscentProfile :> SampledFunction {
		attribute def AscentSample :> SamplePair {
			attribute :>> domainValue: MissionElapsedTimeValue[1];
			attribute :>> rangeValue: PositionAndVelocity[1];
		}
		attribute :>> samples: AscentSample[*] ordered;
	}

	attribute ascentProfile1: AscentProfile {
		doc /* Example ascent profile */
		attribute sample1: AscentSample { :>> domainValue = 0.0 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0, 0, 0) [spatialCF]; :>> velocity = (0, 0, 0) [velocityCF]; } }
		attribute sample2: AscentSample { :>> domainValue = 2.5 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0.01, 0.03, 8.6) [spatialCF]; :>> velocity = (0, 0, 5.5) [velocityCF]; } }
		attribute sample3: AscentSample { :>> domainValue = 5.1 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0.04, 0.12, 18.6) [spatialCF]; :>> velocity = (0.05, 0.03, 25.3) [velocityCF]; } }
		attribute :>> samples = (sample1, sample2, sample3);
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_13_discretely_sampled_function_value.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 1) (end 9 559))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 158))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 1) (end 34 165))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 1) (end 42 280))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 1) (end 50 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 2) (end 53 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 2) (end 54 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 2) (end 67 207))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 2) (end 69 217))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 2) (end 71 225))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_13-Discretely Sampled Function Value' {
    private import SampledFunctions::SampledFunction;
    private import SampledFunctions::SamplePair;
    private import Collections::Array;
    private import ISQ::*;
    private import SI::*;
    private import MeasurementReferences::*;
    private import Time::*;

    attribute def MissionElapsedTimeScale :> TimeScale {
        :>> unit = s;
        attribute :>> definitionalEpoch {
            :>> num = 0;
            :>> definition = "time instant zero at launch";
        }
        attribute definitionalEpochInUTC : Iso8601DateTime;

        // Map the definitional epoch (t = 0) of this scale to a reference epoch expressed in UTC
        // This modeled as a 1D coordinate transformation (translation only)
        attribute :>> transformation : CoordinateFramePlacement {
            :>> source = UTC;
            :>> origin = definitionalEpochInUTC;
            :>> basisDirections = 1 [UTC];
        }
    }

    attribute mets: MissionElapsedTimeScale {
        doc
        /*
		 * Define mission elapsed time scale starting at given UTC date time (in microsecond resolution)
		 */
        :>> definitionalEpochInUTC { :>> val = "2020-08-23T22:42:32.924534Z";}
    }

    attribute def MissionElapsedTimeValue :> TimeInstantValue {
        doc
        /*
		 * Define scalar quantity value type for mission elapsed time
		 */
        :>> mRef = mets;
    }

    attribute spatialCF: CartesianSpatial3dCoordinateFrame[1] {
        doc
        /*
		 * Define Cartesian 3D coordinate systems for position and velocity
		 * Create a velocity coordinate system from the spatial coordinate system through division by second
		 */
        :>> mRefs = (m, m, m);
    }
    attribute velocityCF: CartesianVelocity3dCoordinateFrame[1] = spatialCF/s;

    attribute def PositionAndVelocity {
        attribute position : CartesianPosition3dVector[1];
        attribute velocity : CartesianVelocity3dVector[1];
    }

    attribute def AscentProfile :> SampledFunction {
        attribute def AscentSample :> SamplePair {
            attribute :>> domainValue: MissionElapsedTimeValue[1];
            attribute :>> rangeValue: PositionAndVelocity[1];
        }
        attribute :>> samples: AscentSample[*] ordered;
    }

    attribute ascentProfile1: AscentProfile {
        doc /* Example ascent profile */
        attribute sample1: AscentSample { :>> domainValue = 0.0 [mets]; :>> rangeValue = pv1;
            attribute pv1: PositionAndVelocity {:>> position = (0, 0, 0) [spatialCF]; :>> velocity = (0, 0, 0) [velocityCF]; } }
        attribute sample2: AscentSample { :>> domainValue = 2.5 [mets]; :>> rangeValue = pv1;
            attribute pv1: PositionAndVelocity {:>> position = (0.01, 0.03, 8.6) [spatialCF]; :>> velocity = (0, 0, 5.5) [velocityCF]; } }
        attribute sample3: AscentSample { :>> domainValue = 5.1 [mets]; :>> rangeValue = pv1;
            attribute pv1: PositionAndVelocity {:>> position = (0.04, 0.12, 18.6) [spatialCF]; :>> velocity = (0.05, 0.03, 25.3) [velocityCF]; } }
        attribute :>> samples = (sample1, sample2, sample3);
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6f7275b089fd7dc9d2efbf12ac4bcbde8c108727bd74e83dfbe41713e141419c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (kind "package") (name "15_13-Discretely Sampled Function Value") (declared-name "15_13-Discretely Sampled Function Value"))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::Array"))) (kind "import") (name "Array") (declared-name "Array") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::Array") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (kind "attribute def") (name "AscentProfile") (declared-name "AscentProfile") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "SampledFunction")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (kind "attribute def") (name "AscentSample") (declared-name "AscentSample") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (authored (membership (kind Owning)) (relationships (typing (reference "SamplePair")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind "attribute") (name "samples") (declared-name "samples") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (authored (membership (kind Feature)) (relationships (typing (reference "AscentSample")) (redefinition (reference "samples")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (kind "attribute def") (name "MissionElapsedTimeScale") (declared-name "MissionElapsedTimeScale") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeScale")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (kind "attribute") (name "definitionalEpoch") (declared-name "definitionalEpoch") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalEpoch")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpochInUTC"))) (kind "attribute") (name "definitionalEpochInUTC") (declared-name "definitionalEpochInUTC") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "Iso8601DateTime")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (kind "attribute") (name "transformation") (declared-name "transformation") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "CoordinateFramePlacement")) (redefinition (reference "transformation")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unit")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue"))) (kind "attribute def") (name "MissionElapsedTimeValue") (declared-name "MissionElapsedTimeValue") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity"))) (kind "attribute def") (name "PositionAndVelocity") (declared-name "PositionAndVelocity") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity::position"))) (kind "attribute") (name "position") (declared-name "position") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianPosition3dVector")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianVelocity3dVector")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair"))) (kind "import") (name "SamplePair") (declared-name "SamplePair") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SamplePair") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction"))) (kind "import") (name "SampledFunction") (declared-name "SampledFunction") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SampledFunction") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (kind "attribute def") (name "ascentProfile1") (declared-name "ascentProfile1") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "AscentProfile")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample1"))) (kind "attribute") (name "sample1") (declared-name "sample1") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AscentSample")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample2"))) (kind "attribute") (name "sample2") (declared-name "sample2") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AscentSample")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample3"))) (kind "attribute") (name "sample3") (declared-name "sample3") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AscentSample")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (kind "attribute") (name "samples") (declared-name "samples") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "samples")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (kind "attribute def") (name "mets") (declared-name "mets") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "MissionElapsedTimeScale")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (kind "attribute") (name "definitionalEpochInUTC") (declared-name "definitionalEpochInUTC") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalEpochInUTC")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF"))) (kind "attribute def") (name "spatialCF") (declared-name "spatialCF") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF"))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::velocityCF"))) (kind "attribute def") (name "velocityCF") (declared-name "velocityCF") (parent (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianVelocity3dCoordinateFrame")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "Time::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::Array"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::Array") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (kind featureTyping) (ordinal 0)) (authored-target "SampledFunction") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (kind featureTyping) (ordinal 0)) (authored-target "SamplePair") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentSample") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind redefinition) (ordinal 0)) (authored-target "samples") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeScale") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalEpoch") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpochInUTC"))) (kind featureTyping) (ordinal 0)) (authored-target "Iso8601DateTime") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateFramePlacement") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (kind redefinition) (ordinal 0)) (authored-target "transformation") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (kind redefinition) (ordinal 0)) (authored-target "unit") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity::position"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianPosition3dVector") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::PositionAndVelocity::velocity"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dVector") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SamplePair") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SampledFunction") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentProfile") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample1"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentSample") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample2"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentSample") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::sample3"))) (kind featureTyping) (ordinal 0)) (authored-target "AscentSample") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (kind redefinition) (ordinal 0)) (authored-target "samples") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (kind featureTyping) (ordinal 0)) (authored-target "MissionElapsedTimeScale") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalEpochInUTC") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::velocityCF"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dCoordinateFrame") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::AscentSample"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (target (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::velocityCF")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 16) (end 5 18)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 5 16) (end 5 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 16) (end 4 19)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 4 16) (end 4 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 20)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "Time::*")
        (range (start 7 16) (end 7 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 62 16) (end 62 23)) (probe (position 62 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples"))
        (kind redefinition) (ordinal 0) (authored-target "samples")
        (range (start 62 16) (end 62 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::AscentProfile::samples") (range (start 62 2) (end 62 49)))
        )
      )
    )
    (query (range (start 73 16) (end 73 23)) (probe (position 73 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples"))
        (kind redefinition) (ordinal 0) (authored-target "samples")
        (range (start 73 16) (end 73 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::ascentProfile1::samples") (range (start 73 2) (end 73 54)))
        )
      )
    )
    (query (range (start 10 2) (end 10 10)) (probe (position 10 2))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit"))
        (kind redefinition) (ordinal 0) (authored-target "unit")
        (range (start 10 2) (end 10 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::unit") (range (start 10 2) (end 10 15)))
        )
      )
    )
    (query (range (start 39 3) (end 39 11)) (probe (position 39 3))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 39 3) (end 39 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeValue::mRef") (range (start 39 3) (end 39 19)))
        )
      )
    )
    (query (range (start 48 4) (end 48 13)) (probe (position 48 4))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 48 4) (end 48 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::spatialCF::mRefs") (range (start 48 4) (end 48 26)))
        )
      )
    )
    (query (range (start 19 16) (end 19 30)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation"))
        (kind redefinition) (ordinal 0) (authored-target "transformation")
        (range (start 19 16) (end 19 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::transformation") (range (start 19 2) (end 19 158)))
        )
      )
    )
    (query (range (start 11 16) (end 11 33)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch"))
        (kind redefinition) (ordinal 0) (authored-target "definitionalEpoch")
        (range (start 11 16) (end 11 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::MissionElapsedTimeScale::definitionalEpoch") (range (start 11 2) (end 11 106)))
        )
      )
    )
    (query (range (start 3 16) (end 3 34)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::Array"))
        (kind membershipImport) (ordinal 0) (authored-target "Collections::Array")
        (range (start 3 16) (end 3 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 37)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 6 16) (end 6 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 2) (end 31 28)) (probe (position 31 2))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC"))
        (kind redefinition) (ordinal 0) (authored-target "definitionalEpochInUTC")
        (range (start 31 2) (end 31 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::mets::definitionalEpochInUTC") (range (start 31 2) (end 31 72)))
        )
      )
    )
    (query (range (start 2 16) (end 2 44)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SamplePair"))
        (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SamplePair")
        (range (start 2 16) (end 2 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 49)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_13-Discretely Sampled Function Value::SampledFunction"))
        (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SampledFunction")
        (range (start 1 16) (end 1 49))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
