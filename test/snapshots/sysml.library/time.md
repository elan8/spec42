# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/Time
type=file
~~~
# SOURCE
~~~sysml
standard library package Time {
	doc
	/*
	 * This package specifies concepts to support time-related quantities and metrology, beyond 
	 * the quantities duration and time as defined in [ISO 80000-3]. Representations of the 
	 * Gregorian calendar date and time of day as specified by the [ISO 8601-1] standard are used.
	 */

	private import Occurrences::Occurrence;
	private import ScalarValues::Real;
	private import ScalarValues::Integer;
	private import ScalarValues::Natural;
	private import ScalarValues::String;
	private import Quantities::ScalarQuantityValue;
	private import Quantities::scalarQuantities;
    private import MeasurementReferences::*;
    public import ISQBase::DurationValue;
    public import ISQBase::DurationUnit;
    public import ISQBase::duration;
    public import ISQSpaceTime::TimeValue;
    public import ISQSpaceTime::TimeUnit;
    public import ISQSpaceTime::time;
    
    part universalClock : Clock[1] :> Clocks::universalClock {
   	    doc
	    /*
	     * universalClock is a single Clock that can be used as a default universal time reference.
	     */
    }

	part def Clock :> Clocks::Clock {
		doc
		/*
		 * A Clock provides a currentTime as a TimeInstantValue that advances montonically over its lifetime.
		 */
	
		attribute :>> currentTime : TimeInstantValue;
	}
	
	calc def TimeOf :> Clocks::TimeOf {
		doc
		/*
		 * TimeOf returns a TimeInstantValue for a given Occurrence relative to a given Clock. This TimeInstantValue is the 
		 * time of the start of the Occurrence, which is considered to be synchronized with the snapshot of the Clock with a 
		 * currentTime equal to the returned timeInstant.
		 */
	
		in o : Occurrence[1]; 
		in clock : Clock[1] default localClock;
		return timeInstant : TimeInstantValue[1];
	}

	calc def DurationOf :> Clocks::DurationOf {
		doc
		/*
		 * DurationOf returns the duration of a given Occurrence relative to a given Clock, which is equal to the TimeOf 
		 * the end snapshot of the Occurrence minus the TimeOf its start snapshot.
		 */
	
		in o : Occurrence[1]; 
		in clock : Clock[1] default localClock;
		return duration : DurationValue;
	}
	
    attribute def TimeScale :> IntervalScale {
		doc
		/*
		 * Generic time scale to express a time instant, including a textual definition of the meaning of zero time instant value
		 * 
		 * Attribute definitionalEpoch captures the specification of the time instant with value zero, also known as the (reference) epoch.
		 */
	
		attribute :>> unit: DurationUnit[1];
		attribute definitionalEpoch: DefinitionalQuantityValue[1];
		attribute :>> definitionalQuantityValues = definitionalEpoch;
    }

    attribute def TimeInstantValue :> ScalarQuantityValue {
		doc
		/*
		 * Representation of a time instant quantity
		 *
		 * Also known as instant (of time), or, point in time.
		 */
	
        attribute :>> num: Real[1];
        attribute :>> mRef: TimeScale[1];
    }
    attribute timeInstant: TimeInstantValue :> scalarQuantities;

	abstract attribute def DateTime :> TimeInstantValue {
		doc
		/*
		 * Generic representation of a time instant as a calendar date and time of day
		 */
	}

	abstract attribute def Date :> TimeInstantValue {
		doc
		/*
		 * Generic representation of a time instant as a calendar date
		 */
	}

	abstract attribute def TimeOfDay :> TimeInstantValue {
		doc
		/*
		 * Generic representation of a time instant as a time of day
		 */
	}

	attribute <UTC> 'Coordinated Universal Time' : TimeScale {
		doc
		/*
		 * Representation of the Coordinated Universal Time (UTC) time scale
		 *
		 * UTC is the primary time standard by which the world regulates clocks and time. It is within about 1 second of mean solar time
		 * at 0° longitude and is not adjusted for daylight saving time.
		 * UTC is obtained from International Atomic Time (TAI) by the insertion of leap seconds according to the advice of
		 * the International Earth Rotation and Reference Systems Service (IERS) to ensure approximate agreement
		 * with the time derived from the rotation of the Earth.
		 *
		 * References:
		 * ITU-R TF.460-6 (see https://www.itu.int/rec/R-REC-TF.460/en)
		 * BIPM technical services: Time Metrology (see https://www.bipm.org/en/time-metrology)
		 *
		 * Introductions:
		 * For UTC see https://en.wikipedia.org/wiki/Coordinated_Universal_Time
		 * For TAI see https://en.wikipedia.org/wiki/International_Atomic_Time
		 */
	
		attribute :>> unit = SI::s;
		attribute :>> definitionalEpoch: DefinitionalQuantityValue { :>> num = 0; :>> definition = "UTC epoch at 1 January 1958 at 0 hour 0 minute 0 second"; }
	}

	attribute def UtcTimeInstantValue :> DateTime { 
		:>> mRef = UTC {
			doc
			/*
			 * Representation of a time instant expressed on the Coordinated Universal Time (UTC) time scale
			 */
		} 
	}
	attribute utcTimeInstant: UtcTimeInstantValue :> timeInstant;

	/*
	 * Representations of a Gregorian calendar date and time of day as specified by the ISO 8601-1 standard.
	 *
	 * As explained in ISO 8601-1 clause 4.2.1:
	 * ISO 8601-1 uses the Gregorian calendar for the identification of calendar days.
	 *
	 * The Gregorian calendar provides a time scale consisting of a series of contiguous calendar years,
	 * each identified by a year number represented by an integer, greater than that of the
	 * immediately preceding calendar year by 1. ISO 8601-1 allows the identification of calendar years
	 * by their year number for years both before and after the introduction of the Gregorian calendar.
	 *
	 * The Gregorian calendar distinguishes common years of 365 consecutive calendar days and leap years
	 * of 366 consecutive calendar days.
	 *
	 * In the Gregorian calendar each calendar year is divided into 12 sequential calendar months,
	 * each consisting of a specific number of calendar days in the range 28 to 31. Usage of the Gregorian calendar
	 * for identifying dates preceding its introduction (15 October 1582) should only be by mutual agreement
	 * of the communicating partners.
	 *
	 * Reference: ISO 8601-1:2019 (First edition)
	 * "Date and time — Representations for information interchange — Part 1: Basic rules"
	 * (see https://www.iso.org/standard/70907.html)
	 */

	attribute def Iso8601DateTimeEncoding :> String {
	    doc
	    /*
	     * Extended string encoding of an ISO 8601-1 date and time
	     *
	     * The format of the string must comply with the following EBNF production:
	     * ['+' | '-'] YYYY '-' MM '-' DD 'T' hh ':' mm ':' ss ['.' fff [fff]] ('Z' | timezoneOffset )
	     * where:
	     *   YYYY is 4-or-more-digit year number, which can be negative for years before 0000;
	     *   MM is 2-digit month in year number, in which 01 is January, 02 is February, ..., 12 is December;
	     *   DD is 2-digit day in month number in range 01 to 28, 29, 30, 31 depending on month and leap year;
	     *   hh is 2-digit hour in day number in range 00 to 23;
	     *   mm is 2-digit minute in hour in range 00 to 59;
	     *   ss is 2-digit second in minute in range 00 to 60, in  in case of leap second;
	     *   ['.' fff [fff]] is an optional 3-digit millisecond or 6-digit microsecond fraction;
	     *   timezoneOffset is ('+' | '-') hhOffset ':' mmOffset, denoting the local timezone hour and minute offset w.r.t. UTC,
	     *   in which '+' specifies an offset ahead of UTC and '-' specifies an offset behind UTC;
	     *
	     * Note 1: All components are expressed with leading zeros.
	     * Note 2: 'Z' instead of timezoneOffset denotes a UTC time, i.e. zero time offset.
	     * Note 3: The ss value may only be 60 when a leap second is inserted.
	     *
	     * Examples of such a date and time value are:
	     * 2021-08-30T12:30:24Z (UTC date and time with second precision)
	     * 2018-01-23T23:14:44.304827Z (UTC date and time with microsecond precision)
	     * 1969-07-20T20:17:00Z (UTC date and time with second precision)
	     * 1969-07-20T15:17:00-05:00 (local date and time with second precision for a timezone 5 hour behind UTC)
	     * 1969-07-20T22:17:00+02:00 (local date and time with second precision for a timezone 2 hour ahead of UTC)
	     */
    }

    attribute def Iso8601DateTime :> UtcTimeInstantValue {
		doc
		/*
	     * Representation of an ISO 8601-1 date and time in extended string format
		 */
	
    	attribute val: Iso8601DateTimeEncoding;
    	attribute :>> num = getElapsedUtcTime(val);
    	private calc getElapsedUtcTime {
    		in iso8601DateTime: Iso8601DateTimeEncoding;
    		/* Return the number of seconds elapsed since the UTC epoch. 
    		 * Can be negative when the date and time is earlier than the epoch.
    		 */
    		return : Real;
    	}
    }

    attribute def Iso8601DateTimeStructure :> UtcTimeInstantValue {
		doc
		/*
	     * Representation of an ISO 8601 date and time with explicit date and time component attributes
	     *
	     * The total time offset is equal to the summation of hourOffset and minuteOffset.
		 */
	
    	attribute year: Integer;
    	attribute month: Natural;
    	attribute day: Natural;
    	attribute hour: Natural;
    	attribute minute: Natural;
    	attribute second: Natural;
    	attribute microsecond: Natural;
    	attribute hourOffset: Integer;
    	attribute minuteOffset: Integer;
    	attribute :>> num = getElapsedUtcTime(year, month, day, hour, minute, second, microsecond, hourOffset, minuteOffset);
    	private calc getElapsedUtcTime {
    		in year: Integer; 
    		in month: Natural; 
    		in day: Natural;
    		in hour: Natural;
    		in minute: Natural;
    		in second: Natural;
    		in microsecond: Natural;
    		in hourOffset: Integer;
    		in minuteOffest: Integer;
    		return : Real;
    	}
    }

	calc convertIso8601DateTimeToStructure {
	    doc
	    /*
		 * Calculation to convert an ISO 8601 date and time instant from string to component structure representation
	     */
    
		in iso8601DateTime: Iso8601DateTime;
		/* Parse ISO 8601 string encoding to date and time components */
		return : Iso8601DateTimeStructure;
	}

	calc convertIso8601StructureToDateTime {
		doc
		/*
		 * Calculation to convert an ISO 8601 date and time instant from component structure to string representation
		 */
	
		in iso8601DateTimeStructure: Iso8601DateTimeStructure;
		attribute x: Iso8601DateTime;
		/* Concatenate ISO 8601 date and time components to string 
		 *     year-month-dayThour:minute:second±hourOffset:minuteOffset
		 */
		return : Iso8601DateTime;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "time.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 18) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 18) (end 17 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 18) (end 18 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 18) (end 19 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 18) (end 20 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 18) (end 21 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 38) (end 23 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 30 19) (end 30 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 30) (end 36 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 2) (end 47 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 2) (end 48 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 2) (end 59 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 2) (end 60 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 64 4) (end 64 499))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 2) (end 73 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 132 2) (end 132 153))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "19134c557566e1b4472dec4568555cc495c7faf8be3302eb528e68c4aa0b5bcf") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Time"))) (kind "package") (name "Time") (declared-name "Time"))
    (element (id (node (document "d0") (qualified-name "Time::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::Clock"))) (kind "part def") (name "Clock") (declared-name "Clock") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Clocks::Clock")))))
    (element (id (node (document "d0") (qualified-name "Time::Clock::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::Clock"))))
    (element (id (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (kind "attribute") (name "currentTime") (declared-name "currentTime") (parent (node (document "d0") (qualified-name "Time::Clock"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeInstantValue")) (typing (reference "TimeInstantValue")) (redefinition (reference "currentTime")))))
    (element (id (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (kind "attribute def") (name "Coordinated Universal Time") (declared-name "Coordinated Universal Time") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeScale")))))
    (element (id (node (document "d0") (qualified-name "Time::Coordinated Universal Time::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))))
    (element (id (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (kind "attribute") (name "definitionalEpoch") (declared-name "definitionalEpoch") (parent (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue")) (redefinition (reference "definitionalEpoch")))))
    (element (id (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (parent (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unit")))))
    (element (id (node (document "d0") (qualified-name "Time::Date"))) (kind "attribute def") (name "Date") (declared-name "Date") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "Time::Date::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::Date"))))
    (element (id (node (document "d0") (qualified-name "Time::DateTime"))) (kind "attribute def") (name "DateTime") (declared-name "DateTime") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "Time::DateTime::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::DateTime"))))
    (element (id (node (document "d0") (qualified-name "Time::DurationOf"))) (kind "calc def") (name "DurationOf") (declared-name "DurationOf") (parent (node (document "d0") (qualified-name "Time"))))
    (element (id (node (document "d0") (qualified-name "Time::DurationOf::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::DurationOf"))))
    (element (id (node (document "d0") (qualified-name "Time::DurationOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (parent (node (document "d0") (qualified-name "Time::DurationOf"))) (authored (relationships (typing (reference "clock : Clock[1] default localClock")))))
    (element (id (node (document "d0") (qualified-name "Time::DurationOf::duration"))) (kind "return parameter") (name "duration") (declared-name "duration") (parent (node (document "d0") (qualified-name "Time::DurationOf"))) (authored (relationships (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "Time::DurationOf::o"))) (kind "in out parameter") (name "o") (declared-name "o") (parent (node (document "d0") (qualified-name "Time::DurationOf"))) (authored (relationships (typing (reference "o : Occurrence[1]")))))
    (element (id (node (document "d0") (qualified-name "Time::DurationUnit"))) (kind "import") (name "DurationUnit") (declared-name "DurationUnit") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQBase::DurationUnit") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::DurationValue"))) (kind "import") (name "DurationValue") (declared-name "DurationValue") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQBase::DurationValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::Integer"))) (kind "import") (name "Integer") (declared-name "Integer") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Integer") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (kind "attribute def") (name "Iso8601DateTime") (declared-name "Iso8601DateTime") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "UtcTimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTime::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTime"))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTime::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTime::val"))) (kind "attribute") (name "val") (declared-name "val") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (authored (membership (kind Feature)) (relationships (typing (reference "Iso8601DateTimeEncoding")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (kind "attribute def") (name "Iso8601DateTimeEncoding") (declared-name "Iso8601DateTimeEncoding") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (kind "attribute def") (name "Iso8601DateTimeStructure") (declared-name "Iso8601DateTimeStructure") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "UtcTimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::day"))) (kind "attribute") (name "day") (declared-name "day") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hour"))) (kind "attribute") (name "hour") (declared-name "hour") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hourOffset"))) (kind "attribute") (name "hourOffset") (declared-name "hourOffset") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::microsecond"))) (kind "attribute") (name "microsecond") (declared-name "microsecond") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minute"))) (kind "attribute") (name "minute") (declared-name "minute") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minuteOffset"))) (kind "attribute") (name "minuteOffset") (declared-name "minuteOffset") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::month"))) (kind "attribute") (name "month") (declared-name "month") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::second"))) (kind "attribute") (name "second") (declared-name "second") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::year"))) (kind "attribute") (name "year") (declared-name "year") (parent (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "Time::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (kind "attribute def") (name "TimeInstantValue") (declared-name "TimeInstantValue") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeInstantValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::TimeInstantValue"))))
    (element (id (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeScale")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeOf"))) (kind "calc def") (name "TimeOf") (declared-name "TimeOf") (parent (node (document "d0") (qualified-name "Time"))))
    (element (id (node (document "d0") (qualified-name "Time::TimeOf::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::TimeOf"))))
    (element (id (node (document "d0") (qualified-name "Time::TimeOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (parent (node (document "d0") (qualified-name "Time::TimeOf"))) (authored (relationships (typing (reference "clock : Clock[1] default localClock")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeOf::o"))) (kind "in out parameter") (name "o") (declared-name "o") (parent (node (document "d0") (qualified-name "Time::TimeOf"))) (authored (relationships (typing (reference "o : Occurrence[1]")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeOfDay"))) (kind "attribute def") (name "TimeOfDay") (declared-name "TimeOfDay") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeOfDay::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::TimeOfDay"))))
    (element (id (node (document "d0") (qualified-name "Time::TimeScale"))) (kind "attribute def") (name "TimeScale") (declared-name "TimeScale") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "IntervalScale")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeScale::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::TimeScale"))))
    (element (id (node (document "d0") (qualified-name "Time::TimeScale::definitionalEpoch"))) (kind "attribute") (name "definitionalEpoch") (declared-name "definitionalEpoch") (parent (node (document "d0") (qualified-name "Time::TimeScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues"))) (kind "attribute") (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (parent (node (document "d0") (qualified-name "Time::TimeScale"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalQuantityValues")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (parent (node (document "d0") (qualified-name "Time::TimeScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "DurationUnit")) (redefinition (reference "unit")))))
    (element (id (node (document "d0") (qualified-name "Time::TimeUnit"))) (kind "import") (name "TimeUnit") (declared-name "TimeUnit") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQSpaceTime::TimeUnit") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::TimeValue"))) (kind "import") (name "TimeValue") (declared-name "TimeValue") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQSpaceTime::TimeValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (kind "attribute def") (name "UtcTimeInstantValue") (declared-name "UtcTimeInstantValue") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "DateTime")))))
    (element (id (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))))
    (element (id (node (document "d0") (qualified-name "Time::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time"))))
    (element (id (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure"))) (kind "calc def") (name "convertIso8601DateTimeToStructure") (declared-name "convertIso8601DateTimeToStructure") (parent (node (document "d0") (qualified-name "Time"))))
    (element (id (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure"))) (authored (relationships (typing (reference "Iso8601DateTimeStructure")))))
    (element (id (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure"))))
    (element (id (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::iso8601DateTime"))) (kind "in out parameter") (name "iso8601DateTime") (declared-name "iso8601DateTime") (parent (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure"))) (authored (relationships (typing (reference "Iso8601DateTime")))))
    (element (id (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime"))) (kind "calc def") (name "convertIso8601StructureToDateTime") (declared-name "convertIso8601StructureToDateTime") (parent (node (document "d0") (qualified-name "Time"))))
    (element (id (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime"))) (authored (relationships (typing (reference "Iso8601DateTime")))))
    (element (id (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime"))))
    (element (id (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::iso8601DateTimeStructure"))) (kind "in out parameter") (name "iso8601DateTimeStructure") (declared-name "iso8601DateTimeStructure") (parent (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime"))) (authored (relationships (typing (reference "Iso8601DateTimeStructure")))))
    (element (id (node (document "d0") (qualified-name "Time::duration"))) (kind "import") (name "duration") (declared-name "duration") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQBase::duration") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::scalarQuantities"))) (kind "import") (name "scalarQuantities") (declared-name "scalarQuantities") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::scalarQuantities") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::time"))) (kind "import") (name "time") (declared-name "time") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQSpaceTime::time") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time::timeInstant"))) (kind "attribute def") (name "timeInstant") (declared-name "timeInstant") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "Time::universalClock"))) (kind "part") (name "universalClock") (declared-name "universalClock") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Feature)) (relationships (typing (reference "Clock")) (subsetting (reference "Clocks::universalClock")))))
    (element (id (node (document "d0") (qualified-name "Time::universalClock::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Time::universalClock"))))
    (element (id (node (document "d0") (qualified-name "Time::utcTimeInstant"))) (kind "attribute def") (name "utcTimeInstant") (declared-name "utcTimeInstant") (parent (node (document "d0") (qualified-name "Time"))) (authored (membership (kind Owning)) (relationships (typing (reference "UtcTimeInstantValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Time::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Clock"))) (kind specialization) (ordinal 0)) (authored-target "Clocks::Clock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (kind featureTyping) (ordinal 1)) (authored-target "TimeInstantValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (kind redefinition) (ordinal 0)) (authored-target "currentTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Clock::currentTime")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeScale") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeScale")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalEpoch") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))) (kind redefinition) (ordinal 0)) (authored-target "unit") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Date"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::DateTime"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::DurationOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default localClock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::DurationOf::duration"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::DurationOf::o"))) (kind featureTyping) (ordinal 0)) (authored-target "o : Occurrence[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::DurationUnit"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQBase::DurationUnit") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::DurationValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQBase::DurationValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Integer"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Integer") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (kind featureTyping) (ordinal 0)) (authored-target "UtcTimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::UtcTimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTime::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Iso8601DateTime::num")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTime::val"))) (kind featureTyping) (ordinal 0)) (authored-target "Iso8601DateTimeEncoding") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::String")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (kind featureTyping) (ordinal 0)) (authored-target "UtcTimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::UtcTimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::day"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hour"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hourOffset"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Integer")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::microsecond"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minute"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minuteOffset"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Integer")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::month"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::second"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::year"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Integer")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeScale") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeScale")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeInstantValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default localClock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeOf::o"))) (kind featureTyping) (ordinal 0)) (authored-target "o : Occurrence[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeOfDay"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeScale"))) (kind featureTyping) (ordinal 0)) (authored-target "IntervalScale") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeScale::definitionalEpoch"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalQuantityValues") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::DurationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (kind redefinition) (ordinal 0)) (authored-target "unit") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeScale::unit")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeUnit"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::TimeUnit") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::TimeValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::TimeValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::"))) (kind featureTyping) (ordinal 0)) (authored-target "Iso8601DateTimeStructure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::iso8601DateTime"))) (kind featureTyping) (ordinal 0)) (authored-target "Iso8601DateTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Iso8601DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::"))) (kind featureTyping) (ordinal 0)) (authored-target "Iso8601DateTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Iso8601DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::iso8601DateTimeStructure"))) (kind featureTyping) (ordinal 0)) (authored-target "Iso8601DateTimeStructure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::duration"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQBase::duration") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::scalarQuantities"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::scalarQuantities") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::time"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::time") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::timeInstant"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::universalClock"))) (kind featureTyping) (ordinal 0)) (authored-target "Clock") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::Clock")))))
    (reference (id (source (node (document "d0") (qualified-name "Time::universalClock"))) (kind subsetting) (ordinal 0)) (authored-target "Clocks::universalClock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time::utcTimeInstant"))) (kind featureTyping) (ordinal 0)) (authored-target "UtcTimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time::UtcTimeInstantValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (target (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Clock::currentTime"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (target (node (document "d0") (qualified-name "Time::TimeScale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (target (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))) (target (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Date"))) (target (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Date"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::DateTime"))) (target (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::DateTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::DurationOf::duration"))) (target (node (document "d0") (qualified-name "Time::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::DurationOf::duration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (target (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::Iso8601DateTime::num"))) (target (node (document "d0") (qualified-name "Time::Iso8601DateTime::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTime::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTime::val"))) (target (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTime::val"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (target (node (document "d0") (qualified-name "Time::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeEncoding"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (target (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::day"))) (target (node (document "d0") (qualified-name "Time::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::day"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hour"))) (target (node (document "d0") (qualified-name "Time::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hour"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hourOffset"))) (target (node (document "d0") (qualified-name "Time::Integer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::hourOffset"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::microsecond"))) (target (node (document "d0") (qualified-name "Time::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::microsecond"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minute"))) (target (node (document "d0") (qualified-name "Time::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minute"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minuteOffset"))) (target (node (document "d0") (qualified-name "Time::Integer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::minuteOffset"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::month"))) (target (node (document "d0") (qualified-name "Time::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::month"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))) (target (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::second"))) (target (node (document "d0") (qualified-name "Time::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::year"))) (target (node (document "d0") (qualified-name "Time::Integer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::year"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (target (node (document "d0") (qualified-name "Time::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (target (node (document "d0") (qualified-name "Time::TimeScale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (target (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (target (node (document "d0") (qualified-name "Time::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (target (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeInstantValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::TimeOfDay"))) (target (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeOfDay"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues"))) (target (node (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (target (node (document "d0") (qualified-name "Time::DurationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (target (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::TimeScale::unit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (target (node (document "d0") (qualified-name "Time::DateTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (target (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::"))) (target (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::iso8601DateTime"))) (target (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::convertIso8601DateTimeToStructure::iso8601DateTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::"))) (target (node (document "d0") (qualified-name "Time::Iso8601DateTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::iso8601DateTimeStructure"))) (target (node (document "d0") (qualified-name "Time::Iso8601DateTimeStructure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime::iso8601DateTimeStructure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::timeInstant"))) (target (node (document "d0") (qualified-name "Time::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::timeInstant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::universalClock"))) (target (node (document "d0") (qualified-name "Time::Clock"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::universalClock"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time::utcTimeInstant"))) (target (node (document "d0") (qualified-name "Time::UtcTimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time::utcTimeInstant"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Time::TimeOf")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Time::convertIso8601StructureToDateTime")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 85 22) (end 85 25)) (probe (position 85 22))
      (reference
        (source (document "d0") (qualified-name "Time::TimeInstantValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 85 22) (end 85 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::TimeInstantValue::num") (range (start 85 8) (end 85 35)))
        )
      )
    )
    (query (range (start 207 19) (end 207 22)) (probe (position 207 19))
      (reference
        (source (document "d0") (qualified-name "Time::Iso8601DateTime::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 207 19) (end 207 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::Iso8601DateTime::num") (range (start 207 5) (end 207 48)))
        )
      )
    )
    (query (range (start 234 19) (end 234 22)) (probe (position 234 19))
      (reference
        (source (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 234 19) (end 234 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::Iso8601DateTimeStructure::num") (range (start 234 5) (end 234 122)))
        )
      )
    )
    (query (range (start 72 16) (end 72 20)) (probe (position 72 16))
      (reference
        (source (document "d0") (qualified-name "Time::TimeScale::unit"))
        (kind redefinition) (ordinal 0) (authored-target "unit")
        (range (start 72 16) (end 72 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::TimeScale::unit") (range (start 72 2) (end 72 38)))
        )
      )
    )
    (query (range (start 86 22) (end 86 26)) (probe (position 86 22))
      (reference
        (source (document "d0") (qualified-name "Time::TimeInstantValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 86 22) (end 86 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::TimeInstantValue::mRef") (range (start 86 8) (end 86 41)))
        )
      )
    )
    (query (range (start 131 16) (end 131 20)) (probe (position 131 16))
      (reference
        (source (document "d0") (qualified-name "Time::Coordinated Universal Time::unit"))
        (kind redefinition) (ordinal 0) (authored-target "unit")
        (range (start 131 16) (end 131 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::Coordinated Universal Time::unit") (range (start 131 2) (end 131 29)))
        )
      )
    )
    (query (range (start 23 26) (end 23 31)) (probe (position 23 26))
      (reference
        (source (document "d0") (qualified-name "Time::universalClock"))
        (kind featureTyping) (ordinal 0) (authored-target "Clock")
        (range (start 23 26) (end 23 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::Clock") (range (start 30 1) (end 30 208)))
        )
      )
    )
    (query (range (start 136 2) (end 136 10)) (probe (position 136 2))
      (reference
        (source (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 136 2) (end 136 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::UtcTimeInstantValue::mRef") (range (start 136 2) (end 136 142)))
        )
      )
    )
    (query (range (start 36 16) (end 36 27)) (probe (position 36 16))
      (reference
        (source (document "d0") (qualified-name "Time::Clock::currentTime"))
        (kind redefinition) (ordinal 0) (authored-target "currentTime")
        (range (start 36 16) (end 36 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::Clock::currentTime") (range (start 36 2) (end 36 47)))
        )
      )
    )
    (query (range (start 30 19) (end 30 32)) (probe (position 30 19))
      (reference
        (source (document "d0") (qualified-name "Time::Clock"))
        (kind specialization) (ordinal 0) (authored-target "Clocks::Clock")
        (range (start 30 19) (end 30 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 36 30) (end 36 46)) (probe (position 36 30))
      (reference
        (source (document "d0") (qualified-name "Time::Clock::currentTime"))
        (kind featureTyping) (ordinal 1) (authored-target "TimeInstantValue")
        (range (start 36 30) (end 36 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 18) (end 18 35)) (probe (position 18 18))
      (reference
        (source (document "d0") (qualified-name "Time::duration"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQBase::duration")
        (range (start 18 18) (end 18 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 132 16) (end 132 33)) (probe (position 132 16))
      (reference
        (source (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch"))
        (kind redefinition) (ordinal 0) (authored-target "definitionalEpoch")
        (range (start 132 16) (end 132 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::Coordinated Universal Time::definitionalEpoch") (range (start 132 2) (end 132 153)))
        )
      )
    )
    (query (range (start 9 16) (end 9 34)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Time::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 9 16) (end 9 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 18) (end 21 36)) (probe (position 21 18))
      (reference
        (source (document "d0") (qualified-name "Time::time"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::time")
        (range (start 21 18) (end 21 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 36)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Time::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 12 16) (end 12 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 37)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Time::Integer"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Integer")
        (range (start 10 16) (end 10 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 37)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Time::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 11 16) (end 11 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 19) (end 15 40)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "Time::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 15 19) (end 15 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 18) (end 17 39)) (probe (position 17 18))
      (reference
        (source (document "d0") (qualified-name "Time::DurationUnit"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQBase::DurationUnit")
        (range (start 17 18) (end 17 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 18) (end 16 40)) (probe (position 16 18))
      (reference
        (source (document "d0") (qualified-name "Time::DurationValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQBase::DurationValue")
        (range (start 16 18) (end 16 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 18) (end 20 40)) (probe (position 20 18))
      (reference
        (source (document "d0") (qualified-name "Time::TimeUnit"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::TimeUnit")
        (range (start 20 18) (end 20 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 23 38) (end 23 60)) (probe (position 23 38))
      (reference
        (source (document "d0") (qualified-name "Time::universalClock"))
        (kind subsetting) (ordinal 0) (authored-target "Clocks::universalClock")
        (range (start 23 38) (end 23 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 39)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Time::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 8 16) (end 8 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 18) (end 19 41)) (probe (position 19 18))
      (reference
        (source (document "d0") (qualified-name "Time::TimeValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::TimeValue")
        (range (start 19 18) (end 19 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 74 16) (end 74 42)) (probe (position 74 16))
      (reference
        (source (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues"))
        (kind redefinition) (ordinal 0) (authored-target "definitionalQuantityValues")
        (range (start 74 16) (end 74 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time::TimeScale::definitionalQuantityValues") (range (start 74 2) (end 74 63)))
        )
      )
    )
    (query (range (start 14 16) (end 14 44)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Time::scalarQuantities"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::scalarQuantities")
        (range (start 14 16) (end 14 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 47)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Time::ScalarQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
        (range (start 13 16) (end 13 47))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
