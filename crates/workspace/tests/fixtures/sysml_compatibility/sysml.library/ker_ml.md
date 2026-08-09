# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/KerML
type=file
~~~
# SOURCE
~~~kerml
standard library package KerML {
	doc 
	/*
	 * This package contains a reflective KerML model of the KerML abstract syntax.
	 */
	 
	private import ScalarValues::*;
	public import Kernel::*;
	
	package Root {
		metaclass AnnotatingElement specializes Element {
			derived var feature annotatedElement : Element[1..*] ordered redefines annotatedElement;
			derived composite var feature ownedAnnotatingRelationship : Annotation[0..*] ordered subsets annotation, ownedRelationship;
			derived var feature owningAnnotatingRelationship : Annotation[0..1] subsets owningRelationship, annotation;
			derived var feature annotation : Annotation[0..*] ordered;
		}		
		
		metaclass Annotation specializes Relationship {
			var feature annotatedElement : Element[1..1] redefines target, annotatedElement;
			derived var feature annotatingElement : AnnotatingElement[1..1] redefines source;
			derived var feature owningAnnotatedElement : Element[0..1] subsets annotatedElement, owningRelatedElement;
			derived var feature owningAnnotatingElement : AnnotatingElement[0..1] subsets annotatingElement, owningRelatedElement;
			derived composite var feature ownedAnnotatingElement : AnnotatingElement[0..1] subsets annotatingElement, ownedRelatedElement;
		}		
		
		metaclass Comment specializes AnnotatingElement {
			var feature 'locale' : String[0..1];
			var feature body : String[1..1];
		}		
		
		metaclass Dependency specializes Relationship {
			var feature client : Element[1..*] ordered redefines source;
			var feature supplier : Element[1..*] ordered redefines target;
		}		
		
		metaclass Documentation specializes Comment {
			derived var feature documentedElement : Element[1..1] subsets owner redefines annotatedElement;
		}		
		
		abstract metaclass Element {
			var feature elementId : String[1..1];
			var feature aliasIds : String[0..*] ordered;
			var feature declaredShortName : String[0..1];
			var feature declaredName : String[0..1];
			var feature isImpliedIncluded : Boolean[1..1];
			derived var feature shortName : String[0..1];
			derived var feature name : String[0..1];
			derived var feature qualifiedName : String[0..1];
			derived var feature isLibraryElement : Boolean[1..1];
			
			var feature owningRelationship : Relationship[0..1];
			composite var feature ownedRelationship : Relationship[0..*] ordered;
			derived var feature owningMembership : OwningMembership[0..1] subsets owningRelationship;
			derived var feature owningNamespace : Namespace[0..1];
			derived var feature owner : Element[0..1];
			derived var feature ownedElement : Element[0..*] ordered;
			derived var feature documentation : Documentation[0..*] ordered subsets ownedElement;
			derived composite var feature ownedAnnotation : Annotation[0..*] ordered subsets ownedRelationship;
			derived var feature textualRepresentation : TextualRepresentation[0..*] ordered subsets ownedElement;
		}		
		
		abstract metaclass Import specializes Relationship {
			var feature visibility : VisibilityKind[1..1];
			var feature isRecursive : Boolean[1..1];
			var feature isImportAll : Boolean[1..1];
			
			derived var feature importOwningNamespace : Namespace[1..1] subsets owningRelatedElement redefines source;
			derived var feature importedElement : Element[1..1];
		}		
		
		metaclass Membership specializes Relationship {
			var feature memberShortName : String[0..1];
			var feature memberName : String[0..1];
			var feature visibility : VisibilityKind[1..1];
			derived var feature memberElementId : String[1..1];
			
			var feature memberElement : Element[1..1] redefines target;
			derived var feature membershipOwningNamespace : Namespace[1..1] subsets owningRelatedElement redefines source;
		}		
		
		metaclass MembershipImport specializes Import {
			var feature importedMembership : Membership[1..1] redefines target;
		}		
		
		metaclass Namespace specializes Element {
			derived abstract var feature membership : Membership[0..*] ordered;
			derived composite var feature ownedImport : Import[0..*] ordered subsets ownedRelationship;
			derived var feature 'member' : Element[0..*] ordered;
			derived var feature ownedMember : Element[0..*] ordered subsets 'member';
			derived composite var feature ownedMembership : Membership[0..*] ordered subsets membership, ownedRelationship;
			derived var feature importedMembership : Membership[0..*] ordered subsets membership;
		}		
		
		metaclass NamespaceImport specializes Import {
			var feature importedNamespace : Namespace[1..1] redefines target;
		}		
		
		metaclass OwningMembership specializes Membership {
			derived var feature ownedMemberElementId : String[1..1] redefines memberElementId;
			derived var feature ownedMemberShortName : String[0..1] redefines memberShortName;
			derived var feature ownedMemberName : String[0..1] redefines memberName;
			
			derived composite var feature ownedMemberElement : Element[1..1] subsets ownedRelatedElement redefines memberElement;
		}		
		
		abstract metaclass Relationship specializes Element {
			var feature isImplied : Boolean[1..1];
			
			var feature target : Element[0..*] ordered subsets relatedElement;
			var feature source : Element[0..*] ordered subsets relatedElement;
			var feature owningRelatedElement : Element[0..1] subsets relatedElement;
			composite var feature ownedRelatedElement : Element[0..*] ordered subsets relatedElement;
			derived var feature relatedElement : Element[0..*] ordered nonunique;
		}		
		
		metaclass TextualRepresentation specializes AnnotatingElement {
			var feature 'language' : String[1..1];
			var feature body : String[1..1];
			
			derived var feature representedElement : Element[1..1] subsets owner redefines annotatedElement;
		}		
		
		datatype VisibilityKind {
			member feature 'private' : VisibilityKind[1];
			member feature 'protected' : VisibilityKind[1];
			member feature 'public' : VisibilityKind[1];
		}
		
	}
	
	package Core {
		public import Root::*;
		
		metaclass Classifier specializes Type {
			derived composite var feature ownedSubclassification : Subclassification[0..*] subsets ownedSpecialization;
		}		
		
		metaclass Conjugation specializes Relationship {
			var feature originalType : Type[1..1] redefines target;
			var feature conjugatedType : Type[1..1] redefines source;
			derived var feature owningType : Type[0..1] subsets conjugatedType, owningRelatedElement;
		}		
		
		metaclass CrossSubsetting specializes Subsetting {
			var feature crossedFeature : Feature[1..1] redefines subsettedFeature;
			derived var feature crossingFeature : Feature[1..1] redefines owningFeature, subsettingFeature;
		}		
		
		metaclass Differencing specializes Relationship {
			var feature differencingType : Type[1..1] redefines target;
			derived var feature typeDifferenced : Type[1..1] subsets owningRelatedElement redefines source;
		}		
		
		metaclass Disjoining specializes Relationship {
			var feature typeDisjoined : Type[1..1] redefines source;
			var feature disjoiningType : Type[1..1] redefines target;
			derived var feature owningType : Type[0..1] subsets owningRelatedElement, typeDisjoined;
		}		
		
		metaclass EndFeatureMembership specializes FeatureMembership {
			derived composite var feature ownedMemberFeature : Feature[1..1] redefines ownedMemberFeature;
		}		
		
		metaclass Feature specializes Type {
			var feature isUnique : Boolean[1..1];
			var feature isOrdered : Boolean[1..1];
			var feature isComposite : Boolean[1..1];
			var feature isEnd : Boolean[1..1];
			var feature isDerived : Boolean[1..1];
			var feature isPortion : Boolean[1..1];
			var feature isVariable : Boolean[1..1];
			var feature isConstant : Boolean[1..1];
			var feature direction : FeatureDirectionKind[0..1];
			
			derived var feature owningType : Type[0..1] subsets owningNamespace, featuringType;
			derived var feature 'type' : Type[0..*] ordered;
			derived composite var feature ownedRedefinition : Redefinition[0..*] subsets ownedSubsetting;
			derived composite var feature ownedSubsetting : Subsetting[0..*] subsets ownedSpecialization;
			derived var feature owningFeatureMembership : FeatureMembership[0..1] subsets owningMembership;
			derived var feature endOwningType : Type[0..1] subsets owningType;
			derived composite var feature ownedTyping : FeatureTyping[0..*] ordered subsets ownedSpecialization;
			derived var feature featuringType : Type[0..*] ordered;
			derived composite var feature ownedTypeFeaturing : TypeFeaturing[0..*] ordered subsets ownedRelationship;
			derived var feature chainingFeature : Feature[0..*] ordered nonunique;
			derived composite var feature ownedFeatureInverting : FeatureInverting[0..*] subsets ownedRelationship;
			derived composite var feature ownedFeatureChaining : FeatureChaining[0..*] ordered subsets ownedRelationship;
			derived composite var feature ownedReferenceSubsetting : ReferenceSubsetting[0..1] subsets ownedSubsetting;
			derived var feature featureTarget : Feature[1..1];
			derived var feature crossFeature : Feature[0..1];
			derived composite var feature ownedCrossSubsetting : CrossSubsetting[0..1] subsets ownedSubsetting;
		}		
		
		metaclass FeatureChaining specializes Relationship {
			var feature chainingFeature : Feature[1..1] redefines target;
			derived var feature featureChained : Feature[1..1] subsets owningRelatedElement redefines source;
		}		
		
		datatype FeatureDirectionKind {
			member feature 'in' : FeatureDirectionKind[1];
			member feature 'inout' : FeatureDirectionKind[1];
			member feature 'out' : FeatureDirectionKind[1];
		}
		
		metaclass FeatureInverting specializes Relationship {
			var feature featureInverted : Feature[1..1] redefines source;
			var feature invertingFeature : Feature[1..1] redefines target;
			derived var feature owningFeature : Feature[0..1] subsets featureInverted, owningRelatedElement;
		}		
		
		metaclass FeatureMembership specializes OwningMembership {
			derived var feature owningType : Type[1..1] redefines membershipOwningNamespace;
			derived composite var feature ownedMemberFeature : Feature[1..1] redefines ownedMemberElement;
		}		
		
		metaclass FeatureTyping specializes Specialization {
			var feature typedFeature : Feature[1..1] redefines specific;
			var feature 'type' : Type[1..1] redefines general;
			derived var feature owningFeature : Feature[0..1] subsets typedFeature redefines owningType;
		}		
		
		metaclass Intersecting specializes Relationship {
			var feature intersectingType : Type[1..1] redefines target;
			derived var feature typeIntersected : Type[1..1] subsets owningRelatedElement redefines source;
		}		
		
		metaclass Multiplicity specializes Feature;		
		
		metaclass Redefinition specializes Subsetting {
			var feature redefiningFeature : Feature[1..1] redefines subsettingFeature;
			var feature redefinedFeature : Feature[1..1] redefines subsettedFeature;
		}		
		
		metaclass ReferenceSubsetting specializes Subsetting {
			var feature referencedFeature : Feature[1..1] redefines subsettedFeature;
			derived var feature referencingFeature : Feature[1..1] redefines owningFeature, subsettingFeature;
		}		
		
		metaclass Specialization specializes Relationship {
			var feature general : Type[1..1] redefines target;
			var feature specific : Type[1..1] redefines source;
			derived var feature owningType : Type[0..1] subsets owningRelatedElement, specific;
		}		
		
		metaclass Subclassification specializes Specialization {
			var feature superclassifier : Classifier[1..1] redefines general;
			var feature 'subclassifier' : Classifier[1..1] redefines specific;
			derived var feature owningClassifier : Classifier[0..1] redefines owningType;
		}		
		
		metaclass Subsetting specializes Specialization {
			var feature subsettedFeature : Feature[1..1] redefines general;
			var feature subsettingFeature : Feature[1..1] redefines specific;
			derived var feature owningFeature : Feature[0..1] subsets subsettingFeature redefines owningType;
		}		
		
		metaclass Type specializes Namespace {
			var feature isAbstract : Boolean[1..1];
			var feature isSufficient : Boolean[1..1];
			derived var feature isConjugated : Boolean[1..1];
			
			derived composite var feature ownedSpecialization : Specialization[0..*] ordered subsets ownedRelationship;
			derived composite var feature ownedFeatureMembership : FeatureMembership[0..*] ordered subsets ownedMembership, featureMembership;
			derived var feature 'feature' : Feature[0..*] ordered subsets 'member';
			derived var feature ownedFeature : Feature[0..*] ordered subsets ownedMember;
			derived var feature input : Feature[0..*] ordered subsets directedFeature;
			derived var feature output : Feature[0..*] ordered subsets directedFeature;
			derived var feature inheritedMembership : Membership[0..*] ordered subsets membership;
			derived var feature endFeature : Feature[0..*] ordered subsets 'feature';
			derived var feature ownedEndFeature : Feature[0..*] ordered subsets endFeature, ownedFeature;
			derived composite var feature ownedConjugator : Conjugation[0..1] subsets ownedRelationship;
			derived var feature inheritedFeature : Feature[0..*] ordered subsets 'feature';
			derived var feature 'multiplicity' : Multiplicity[0..1] subsets ownedMember;
			derived var feature unioningType : Type[0..*] ordered;
			derived composite var feature ownedIntersecting : Intersecting[0..*] ordered subsets ownedRelationship;
			derived var feature intersectingType : Type[0..*] ordered;
			derived composite var feature ownedUnioning : Unioning[0..*] ordered subsets ownedRelationship;
			derived composite var feature ownedDisjoining : Disjoining[0..*] subsets ownedRelationship;
			derived var feature featureMembership : FeatureMembership[0..*] ordered;
			derived var feature differencingType : Type[0..*] ordered;
			derived composite var feature ownedDifferencing : Differencing[0..*] ordered subsets ownedRelationship;
			derived var feature directedFeature : Feature[0..*] ordered subsets 'feature';
		}		
		
		metaclass TypeFeaturing specializes Relationship {
			var feature featureOfType : Feature[1..1] redefines source;
			var feature featuringType : Type[1..1] redefines target;
			derived var feature owningFeatureOfType : Feature[0..1] subsets owningRelatedElement, featureOfType;
		}		
		
		metaclass Unioning specializes Relationship {
			var feature unioningType : Type[1..1] redefines target;
			derived var feature typeUnioned : Type[1..1] subsets owningRelatedElement redefines source;
		}		
		
	}
	
	package Kernel {
		public import Core::*;
		
		metaclass Association specializes Classifier, Relationship {
			derived var feature relatedType : Type[0..*] ordered nonunique redefines relatedElement;
			derived var feature sourceType : Type[0..1] subsets relatedType redefines source;
			derived var feature targetType : Type[0..*] subsets relatedType redefines target;
			derived var feature associationEnd : Feature[0..*] redefines endFeature;
		}		
		
		metaclass AssociationStructure specializes Association, Structure;		
		
		metaclass Behavior specializes Class {
			derived var feature 'step' : Step[0..*] subsets 'feature';
			derived var feature parameter : Feature[0..*] ordered redefines directedFeature;
		}		
		
		metaclass BindingConnector specializes Connector;		
		
		metaclass BooleanExpression specializes Expression {
			derived var feature 'predicate' : Predicate[0..1] redefines 'function';
		}		
		
		metaclass Class specializes Classifier;		
		
		metaclass CollectExpression specializes OperatorExpression {
			var feature operator : String[1..1] redefines operator;
		}		
		
		metaclass Connector specializes Feature, Relationship {
			derived var feature relatedFeature : Feature[0..*] ordered nonunique redefines relatedElement;
			derived var feature association : Association[0..*] ordered redefines 'type';
			derived var feature connectorEnd : Feature[0..*] ordered redefines endFeature;
			derived var feature sourceFeature : Feature[0..1] ordered subsets relatedFeature redefines source;
			derived var feature targetFeature : Feature[0..*] ordered subsets relatedFeature redefines target;
			derived var feature defaultFeaturingType : Type[0..1];
		}		
		
		metaclass ConstructorExpression specializes InstantiationExpression;		
		
		metaclass DataType specializes Classifier;		
		
		metaclass ElementFilterMembership specializes OwningMembership {
			derived composite var feature condition : Expression[1..1] redefines ownedMemberElement;
		}		
		
		metaclass Expression specializes Step {
			derived var feature isModelLevelEvaluable : Boolean[1..1];
			
			derived var feature 'function' : Function[0..1] redefines 'behavior';
			derived var feature result : Feature[1..1] subsets output, parameter;
		}		
		
		metaclass FeatureChainExpression specializes OperatorExpression {
			var feature operator : String[1..1] redefines operator;
			
			derived var feature targetFeature : Feature[1..1] subsets 'member';
		}		
		
		metaclass FeatureReferenceExpression specializes Expression {
			derived var feature referent : Feature[1..1] subsets 'member';
		}		
		
		metaclass FeatureValue specializes OwningMembership {
			var feature isInitial : Boolean[1..1];
			var feature isDefault : Boolean[1..1];
			
			derived var feature featureWithValue : Feature[1..1] subsets membershipOwningNamespace;
			derived composite var feature value : Expression[1..1] redefines ownedMemberElement;
		}		
		
		metaclass Flow specializes Connector, Step {
			derived var feature payloadType : Classifier[0..*] ordered nonunique;
			derived var feature targetInputFeature : Feature[0..1] ordered nonunique;
			derived var feature sourceOutputFeature : Feature[0..1] ordered nonunique;
			derived var feature flowEnd : FlowEnd[0..2] ordered subsets connectorEnd;
			derived var feature payloadFeature : PayloadFeature[0..1] subsets ownedFeature;
			derived var feature 'interaction' : Interaction[0..*] ordered redefines association, 'behavior';
		}		
		
		metaclass FlowEnd specializes Feature;		
		
		metaclass Function specializes Behavior {
			derived var feature isModelLevelEvaluable : Boolean[1..1];
			
			derived var feature expression : Expression[0..*] subsets 'step';
			derived var feature result : Feature[1..1] subsets output, parameter;
		}		
		
		metaclass IndexExpression specializes OperatorExpression {
			var feature operator : String[1..1] redefines operator;
		}		
		
		abstract metaclass InstantiationExpression specializes Expression {
			derived var feature argument : Expression[0..*] ordered;
			derived var feature instantiatedType : Type[1..1] subsets 'member';
		}		
		
		metaclass Interaction specializes Association, Behavior;		
		
		metaclass Invariant specializes BooleanExpression {
			var feature isNegated : Boolean[1..1];
		}		
		
		metaclass InvocationExpression specializes InstantiationExpression;		
		
		metaclass LibraryPackage specializes Package {
			var feature isStandard : Boolean[1..1];
		}		
		
		metaclass LiteralBoolean specializes LiteralExpression {
			var feature value : Boolean[1..1];
		}		
		
		metaclass LiteralExpression specializes Expression;		
		
		metaclass LiteralInfinity specializes LiteralExpression;		
		
		metaclass LiteralInteger specializes LiteralExpression {
			var feature value : Integer[1..1];
		}		
		
		metaclass LiteralRational specializes LiteralExpression {
			var feature value : Rational[1..1];
		}		
		
		metaclass LiteralString specializes LiteralExpression {
			var feature value : String[1..1];
		}		
		
		metaclass Metaclass specializes Structure;		
		
		metaclass MetadataAccessExpression specializes Expression {
			derived var feature referencedElement : Element[1..1] subsets 'member';
		}		
		
		metaclass MetadataFeature specializes AnnotatingElement, Feature {
			derived var feature 'metaclass' : Metaclass[0..1] subsets 'type';
		}		
		
		metaclass MultiplicityRange specializes Multiplicity {
			derived var feature lowerBound : Expression[0..1] subsets bound;
			derived var feature upperBound : Expression[1..1] subsets bound;
			derived var feature bound : Expression[1..2] ordered subsets ownedMember;
		}		
		
		metaclass NullExpression specializes Expression;		
		
		metaclass OperatorExpression specializes InvocationExpression {
			var feature operator : String[1..1];
		}		
		
		metaclass Package specializes Namespace {
			derived var feature filterCondition : Expression[0..*] ordered subsets ownedMember;
		}		
		
		metaclass ParameterMembership specializes FeatureMembership {
			derived composite var feature ownedMemberParameter : Feature[1..1] redefines ownedMemberFeature;
		}		
		
		metaclass PayloadFeature specializes Feature;		
		
		metaclass Predicate specializes Function;		
		
		metaclass ResultExpressionMembership specializes FeatureMembership {
			derived composite var feature ownedResultExpression : Expression[1..1] redefines ownedMemberFeature;
		}		
		
		metaclass ReturnParameterMembership specializes ParameterMembership;		
		
		metaclass SelectExpression specializes OperatorExpression {
			var feature operator : String[1..1] redefines operator;
		}		
		
		metaclass Step specializes Feature {
			derived var feature 'behavior' : Behavior[0..*] ordered subsets 'type';
			derived var feature parameter : Feature[0..*] ordered redefines directedFeature;
		}		
		
		metaclass Structure specializes Class;		
		
		metaclass Succession specializes Connector;		
		
		metaclass SuccessionFlow specializes Succession, Flow;		
		
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Comma,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMetaclass,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwAbstract,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,UnrestrictedName,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwDatatype,Ident,OpenCurly,
KwMember,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwMember,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwMember,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwDatatype,Ident,OpenCurly,
KwMember,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwMember,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwMember,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,Semicolon,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,UnrestrictedName,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,UnrestrictedName,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,UnrestrictedName,Semicolon,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,UnrestrictedName,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,UnrestrictedName,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,UnrestrictedName,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,UnrestrictedName,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,UnrestrictedName,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,UnrestrictedName,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,UnrestrictedName,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,Comma,UnrestrictedName,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,UnrestrictedName,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,UnrestrictedName,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,UnrestrictedName,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,UnrestrictedName,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwComposite,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,UnrestrictedName,Semicolon,
KwDerived,KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'KerML'
    (documentation)
    (import_decl private 'ScalarValues::*')
    (import_decl public 'Kernel::*')
    (package_def 'Root'
      (metaclass_def 'AnnotatingElement' :> 'Element'
        (feature_def derived var 'annotatedElement' : 'Element' multiplicity :>> 'annotatedElement' ordered)
        (feature_def derived composite var 'ownedAnnotatingRelationship' : 'Annotation' multiplicity :> 'annotation', 'ownedRelationship' ordered)
        (feature_def derived var 'owningAnnotatingRelationship' : 'Annotation' multiplicity :> 'owningRelationship', 'annotation')
        (feature_def derived var 'annotation' : 'Annotation' multiplicity ordered))
      (metaclass_def 'Annotation' :> 'Relationship'
        (feature_def var 'annotatedElement' : 'Element' multiplicity :>> 'target', 'annotatedElement')
        (feature_def derived var 'annotatingElement' : 'AnnotatingElement' multiplicity :>> 'source')
        (feature_def derived var 'owningAnnotatedElement' : 'Element' multiplicity :> 'annotatedElement', 'owningRelatedElement')
        (feature_def derived var 'owningAnnotatingElement' : 'AnnotatingElement' multiplicity :> 'annotatingElement', 'owningRelatedElement')
        (feature_def derived composite var 'ownedAnnotatingElement' : 'AnnotatingElement' multiplicity :> 'annotatingElement', 'ownedRelatedElement'))
      (metaclass_def 'Comment' :> 'AnnotatingElement'
        (feature_def var ''locale'' : 'String' multiplicity)
        (feature_def var 'body' : 'String' multiplicity))
      (metaclass_def 'Dependency' :> 'Relationship'
        (feature_def var 'client' : 'Element' multiplicity :>> 'source' ordered)
        (feature_def var 'supplier' : 'Element' multiplicity :>> 'target' ordered))
      (metaclass_def 'Documentation' :> 'Comment'
        (feature_def derived var 'documentedElement' : 'Element' multiplicity :> 'owner' :>> 'annotatedElement'))
      (metaclass_def abstract 'Element'
        (feature_def var 'elementId' : 'String' multiplicity)
        (feature_def var 'aliasIds' : 'String' multiplicity ordered)
        (feature_def var 'declaredShortName' : 'String' multiplicity)
        (feature_def var 'declaredName' : 'String' multiplicity)
        (feature_def var 'isImpliedIncluded' : 'Boolean' multiplicity)
        (feature_def derived var 'shortName' : 'String' multiplicity)
        (feature_def derived var 'name' : 'String' multiplicity)
        (feature_def derived var 'qualifiedName' : 'String' multiplicity)
        (feature_def derived var 'isLibraryElement' : 'Boolean' multiplicity)
        (feature_def var 'owningRelationship' : 'Relationship' multiplicity)
        (feature_def composite var 'ownedRelationship' : 'Relationship' multiplicity ordered)
        (feature_def derived var 'owningMembership' : 'OwningMembership' multiplicity :> 'owningRelationship')
        (feature_def derived var 'owningNamespace' : 'Namespace' multiplicity)
        (feature_def derived var 'owner' : 'Element' multiplicity)
        (feature_def derived var 'ownedElement' : 'Element' multiplicity ordered)
        (feature_def derived var 'documentation' : 'Documentation' multiplicity :> 'ownedElement' ordered)
        (feature_def derived composite var 'ownedAnnotation' : 'Annotation' multiplicity :> 'ownedRelationship' ordered)
        (feature_def derived var 'textualRepresentation' : 'TextualRepresentation' multiplicity :> 'ownedElement' ordered))
      (metaclass_def abstract 'Import' :> 'Relationship'
        (feature_def var 'visibility' : 'VisibilityKind' multiplicity)
        (feature_def var 'isRecursive' : 'Boolean' multiplicity)
        (feature_def var 'isImportAll' : 'Boolean' multiplicity)
        (feature_def derived var 'importOwningNamespace' : 'Namespace' multiplicity :> 'owningRelatedElement' :>> 'source')
        (feature_def derived var 'importedElement' : 'Element' multiplicity))
      (metaclass_def 'Membership' :> 'Relationship'
        (feature_def var 'memberShortName' : 'String' multiplicity)
        (feature_def var 'memberName' : 'String' multiplicity)
        (feature_def var 'visibility' : 'VisibilityKind' multiplicity)
        (feature_def derived var 'memberElementId' : 'String' multiplicity)
        (feature_def var 'memberElement' : 'Element' multiplicity :>> 'target')
        (feature_def derived var 'membershipOwningNamespace' : 'Namespace' multiplicity :> 'owningRelatedElement' :>> 'source'))
      (metaclass_def 'MembershipImport' :> 'Import'
        (feature_def var 'importedMembership' : 'Membership' multiplicity :>> 'target'))
      (metaclass_def 'Namespace' :> 'Element'
        (feature_def derived abstract var 'membership' : 'Membership' multiplicity ordered)
        (feature_def derived composite var 'ownedImport' : 'Import' multiplicity :> 'ownedRelationship' ordered)
        (feature_def derived var ''member'' : 'Element' multiplicity ordered)
        (feature_def derived var 'ownedMember' : 'Element' multiplicity :> ''member'' ordered)
        (feature_def derived composite var 'ownedMembership' : 'Membership' multiplicity :> 'membership', 'ownedRelationship' ordered)
        (feature_def derived var 'importedMembership' : 'Membership' multiplicity :> 'membership' ordered))
      (metaclass_def 'NamespaceImport' :> 'Import'
        (feature_def var 'importedNamespace' : 'Namespace' multiplicity :>> 'target'))
      (metaclass_def 'OwningMembership' :> 'Membership'
        (feature_def derived var 'ownedMemberElementId' : 'String' multiplicity :>> 'memberElementId')
        (feature_def derived var 'ownedMemberShortName' : 'String' multiplicity :>> 'memberShortName')
        (feature_def derived var 'ownedMemberName' : 'String' multiplicity :>> 'memberName')
        (feature_def derived composite var 'ownedMemberElement' : 'Element' multiplicity :> 'ownedRelatedElement' :>> 'memberElement'))
      (metaclass_def abstract 'Relationship' :> 'Element'
        (feature_def var 'isImplied' : 'Boolean' multiplicity)
        (feature_def var 'target' : 'Element' multiplicity :> 'relatedElement' ordered)
        (feature_def var 'source' : 'Element' multiplicity :> 'relatedElement' ordered)
        (feature_def var 'owningRelatedElement' : 'Element' multiplicity :> 'relatedElement')
        (feature_def composite var 'ownedRelatedElement' : 'Element' multiplicity :> 'relatedElement' ordered)
        (feature_def derived var 'relatedElement' : 'Element' multiplicity ordered nonunique))
      (metaclass_def 'TextualRepresentation' :> 'AnnotatingElement'
        (feature_def var ''language'' : 'String' multiplicity)
        (feature_def var 'body' : 'String' multiplicity)
        (feature_def derived var 'representedElement' : 'Element' multiplicity :> 'owner' :>> 'annotatedElement'))
      (datatype_def 'VisibilityKind'
        (feature_def member ''private'' : 'VisibilityKind' multiplicity)
        (feature_def member ''protected'' : 'VisibilityKind' multiplicity)
        (feature_def member ''public'' : 'VisibilityKind' multiplicity)))
    (package_def 'Core'
      (import_decl public 'Root::*')
      (metaclass_def 'Classifier' :> 'Type'
        (feature_def derived composite var 'ownedSubclassification' : 'Subclassification' multiplicity :> 'ownedSpecialization'))
      (metaclass_def 'Conjugation' :> 'Relationship'
        (feature_def var 'originalType' : 'Type' multiplicity :>> 'target')
        (feature_def var 'conjugatedType' : 'Type' multiplicity :>> 'source')
        (feature_def derived var 'owningType' : 'Type' multiplicity :> 'conjugatedType', 'owningRelatedElement'))
      (metaclass_def 'CrossSubsetting' :> 'Subsetting'
        (feature_def var 'crossedFeature' : 'Feature' multiplicity :>> 'subsettedFeature')
        (feature_def derived var 'crossingFeature' : 'Feature' multiplicity :>> 'owningFeature', 'subsettingFeature'))
      (metaclass_def 'Differencing' :> 'Relationship'
        (feature_def var 'differencingType' : 'Type' multiplicity :>> 'target')
        (feature_def derived var 'typeDifferenced' : 'Type' multiplicity :> 'owningRelatedElement' :>> 'source'))
      (metaclass_def 'Disjoining' :> 'Relationship'
        (feature_def var 'typeDisjoined' : 'Type' multiplicity :>> 'source')
        (feature_def var 'disjoiningType' : 'Type' multiplicity :>> 'target')
        (feature_def derived var 'owningType' : 'Type' multiplicity :> 'owningRelatedElement', 'typeDisjoined'))
      (metaclass_def 'EndFeatureMembership' :> 'FeatureMembership'
        (feature_def derived composite var 'ownedMemberFeature' : 'Feature' multiplicity :>> 'ownedMemberFeature'))
      (metaclass_def 'Feature' :> 'Type'
        (feature_def var 'isUnique' : 'Boolean' multiplicity)
        (feature_def var 'isOrdered' : 'Boolean' multiplicity)
        (feature_def var 'isComposite' : 'Boolean' multiplicity)
        (feature_def var 'isEnd' : 'Boolean' multiplicity)
        (feature_def var 'isDerived' : 'Boolean' multiplicity)
        (feature_def var 'isPortion' : 'Boolean' multiplicity)
        (feature_def var 'isVariable' : 'Boolean' multiplicity)
        (feature_def var 'isConstant' : 'Boolean' multiplicity)
        (feature_def var 'direction' : 'FeatureDirectionKind' multiplicity)
        (feature_def derived var 'owningType' : 'Type' multiplicity :> 'owningNamespace', 'featuringType')
        (feature_def derived var ''type'' : 'Type' multiplicity ordered)
        (feature_def derived composite var 'ownedRedefinition' : 'Redefinition' multiplicity :> 'ownedSubsetting')
        (feature_def derived composite var 'ownedSubsetting' : 'Subsetting' multiplicity :> 'ownedSpecialization')
        (feature_def derived var 'owningFeatureMembership' : 'FeatureMembership' multiplicity :> 'owningMembership')
        (feature_def derived var 'endOwningType' : 'Type' multiplicity :> 'owningType')
        (feature_def derived composite var 'ownedTyping' : 'FeatureTyping' multiplicity :> 'ownedSpecialization' ordered)
        (feature_def derived var 'featuringType' : 'Type' multiplicity ordered)
        (feature_def derived composite var 'ownedTypeFeaturing' : 'TypeFeaturing' multiplicity :> 'ownedRelationship' ordered)
        (feature_def derived var 'chainingFeature' : 'Feature' multiplicity ordered nonunique)
        (feature_def derived composite var 'ownedFeatureInverting' : 'FeatureInverting' multiplicity :> 'ownedRelationship')
        (feature_def derived composite var 'ownedFeatureChaining' : 'FeatureChaining' multiplicity :> 'ownedRelationship' ordered)
        (feature_def derived composite var 'ownedReferenceSubsetting' : 'ReferenceSubsetting' multiplicity :> 'ownedSubsetting')
        (feature_def derived var 'featureTarget' : 'Feature' multiplicity)
        (feature_def derived var 'crossFeature' : 'Feature' multiplicity)
        (feature_def derived composite var 'ownedCrossSubsetting' : 'CrossSubsetting' multiplicity :> 'ownedSubsetting'))
      (metaclass_def 'FeatureChaining' :> 'Relationship'
        (feature_def var 'chainingFeature' : 'Feature' multiplicity :>> 'target')
        (feature_def derived var 'featureChained' : 'Feature' multiplicity :> 'owningRelatedElement' :>> 'source'))
      (datatype_def 'FeatureDirectionKind'
        (feature_def member ''in'' : 'FeatureDirectionKind' multiplicity)
        (feature_def member ''inout'' : 'FeatureDirectionKind' multiplicity)
        (feature_def member ''out'' : 'FeatureDirectionKind' multiplicity))
      (metaclass_def 'FeatureInverting' :> 'Relationship'
        (feature_def var 'featureInverted' : 'Feature' multiplicity :>> 'source')
        (feature_def var 'invertingFeature' : 'Feature' multiplicity :>> 'target')
        (feature_def derived var 'owningFeature' : 'Feature' multiplicity :> 'featureInverted', 'owningRelatedElement'))
      (metaclass_def 'FeatureMembership' :> 'OwningMembership'
        (feature_def derived var 'owningType' : 'Type' multiplicity :>> 'membershipOwningNamespace')
        (feature_def derived composite var 'ownedMemberFeature' : 'Feature' multiplicity :>> 'ownedMemberElement'))
      (metaclass_def 'FeatureTyping' :> 'Specialization'
        (feature_def var 'typedFeature' : 'Feature' multiplicity :>> 'specific')
        (feature_def var ''type'' : 'Type' multiplicity :>> 'general')
        (feature_def derived var 'owningFeature' : 'Feature' multiplicity :> 'typedFeature' :>> 'owningType'))
      (metaclass_def 'Intersecting' :> 'Relationship'
        (feature_def var 'intersectingType' : 'Type' multiplicity :>> 'target')
        (feature_def derived var 'typeIntersected' : 'Type' multiplicity :> 'owningRelatedElement' :>> 'source'))
      (metaclass_def 'Multiplicity' :> 'Feature')
      (metaclass_def 'Redefinition' :> 'Subsetting'
        (feature_def var 'redefiningFeature' : 'Feature' multiplicity :>> 'subsettingFeature')
        (feature_def var 'redefinedFeature' : 'Feature' multiplicity :>> 'subsettedFeature'))
      (metaclass_def 'ReferenceSubsetting' :> 'Subsetting'
        (feature_def var 'referencedFeature' : 'Feature' multiplicity :>> 'subsettedFeature')
        (feature_def derived var 'referencingFeature' : 'Feature' multiplicity :>> 'owningFeature', 'subsettingFeature'))
      (metaclass_def 'Specialization' :> 'Relationship'
        (feature_def var 'general' : 'Type' multiplicity :>> 'target')
        (feature_def var 'specific' : 'Type' multiplicity :>> 'source')
        (feature_def derived var 'owningType' : 'Type' multiplicity :> 'owningRelatedElement', 'specific'))
      (metaclass_def 'Subclassification' :> 'Specialization'
        (feature_def var 'superclassifier' : 'Classifier' multiplicity :>> 'general')
        (feature_def var ''subclassifier'' : 'Classifier' multiplicity :>> 'specific')
        (feature_def derived var 'owningClassifier' : 'Classifier' multiplicity :>> 'owningType'))
      (metaclass_def 'Subsetting' :> 'Specialization'
        (feature_def var 'subsettedFeature' : 'Feature' multiplicity :>> 'general')
        (feature_def var 'subsettingFeature' : 'Feature' multiplicity :>> 'specific')
        (feature_def derived var 'owningFeature' : 'Feature' multiplicity :> 'subsettingFeature' :>> 'owningType'))
      (metaclass_def 'Type' :> 'Namespace'
        (feature_def var 'isAbstract' : 'Boolean' multiplicity)
        (feature_def var 'isSufficient' : 'Boolean' multiplicity)
        (feature_def derived var 'isConjugated' : 'Boolean' multiplicity)
        (feature_def derived composite var 'ownedSpecialization' : 'Specialization' multiplicity :> 'ownedRelationship' ordered)
        (feature_def derived composite var 'ownedFeatureMembership' : 'FeatureMembership' multiplicity :> 'ownedMembership', 'featureMembership' ordered)
        (feature_def derived var ''feature'' : 'Feature' multiplicity :> ''member'' ordered)
        (feature_def derived var 'ownedFeature' : 'Feature' multiplicity :> 'ownedMember' ordered)
        (feature_def derived var 'input' : 'Feature' multiplicity :> 'directedFeature' ordered)
        (feature_def derived var 'output' : 'Feature' multiplicity :> 'directedFeature' ordered)
        (feature_def derived var 'inheritedMembership' : 'Membership' multiplicity :> 'membership' ordered)
        (feature_def derived var 'endFeature' : 'Feature' multiplicity :> ''feature'' ordered)
        (feature_def derived var 'ownedEndFeature' : 'Feature' multiplicity :> 'endFeature', 'ownedFeature' ordered)
        (feature_def derived composite var 'ownedConjugator' : 'Conjugation' multiplicity :> 'ownedRelationship')
        (feature_def derived var 'inheritedFeature' : 'Feature' multiplicity :> ''feature'' ordered)
        (feature_def derived var ''multiplicity'' : 'Multiplicity' multiplicity :> 'ownedMember')
        (feature_def derived var 'unioningType' : 'Type' multiplicity ordered)
        (feature_def derived composite var 'ownedIntersecting' : 'Intersecting' multiplicity :> 'ownedRelationship' ordered)
        (feature_def derived var 'intersectingType' : 'Type' multiplicity ordered)
        (feature_def derived composite var 'ownedUnioning' : 'Unioning' multiplicity :> 'ownedRelationship' ordered)
        (feature_def derived composite var 'ownedDisjoining' : 'Disjoining' multiplicity :> 'ownedRelationship')
        (feature_def derived var 'featureMembership' : 'FeatureMembership' multiplicity ordered)
        (feature_def derived var 'differencingType' : 'Type' multiplicity ordered)
        (feature_def derived composite var 'ownedDifferencing' : 'Differencing' multiplicity :> 'ownedRelationship' ordered)
        (feature_def derived var 'directedFeature' : 'Feature' multiplicity :> ''feature'' ordered))
      (metaclass_def 'TypeFeaturing' :> 'Relationship'
        (feature_def var 'featureOfType' : 'Feature' multiplicity :>> 'source')
        (feature_def var 'featuringType' : 'Type' multiplicity :>> 'target')
        (feature_def derived var 'owningFeatureOfType' : 'Feature' multiplicity :> 'owningRelatedElement', 'featureOfType'))
      (metaclass_def 'Unioning' :> 'Relationship'
        (feature_def var 'unioningType' : 'Type' multiplicity :>> 'target')
        (feature_def derived var 'typeUnioned' : 'Type' multiplicity :> 'owningRelatedElement' :>> 'source')))
    (package_def 'Kernel'
      (import_decl public 'Core::*')
      (metaclass_def 'Association' :> 'Classifier', 'Relationship'
        (feature_def derived var 'relatedType' : 'Type' multiplicity :>> 'relatedElement' ordered nonunique)
        (feature_def derived var 'sourceType' : 'Type' multiplicity :> 'relatedType' :>> 'source')
        (feature_def derived var 'targetType' : 'Type' multiplicity :> 'relatedType' :>> 'target')
        (feature_def derived var 'associationEnd' : 'Feature' multiplicity :>> 'endFeature'))
      (metaclass_def 'AssociationStructure' :> 'Association', 'Structure')
      (metaclass_def 'Behavior' :> 'Class'
        (feature_def derived var ''step'' : 'Step' multiplicity :> ''feature'')
        (feature_def derived var 'parameter' : 'Feature' multiplicity :>> 'directedFeature' ordered))
      (metaclass_def 'BindingConnector' :> 'Connector')
      (metaclass_def 'BooleanExpression' :> 'Expression'
        (feature_def derived var ''predicate'' : 'Predicate' multiplicity :>> ''function''))
      (metaclass_def 'Class' :> 'Classifier')
      (metaclass_def 'CollectExpression' :> 'OperatorExpression'
        (feature_def var 'operator' : 'String' multiplicity :>> 'operator'))
      (metaclass_def 'Connector' :> 'Feature', 'Relationship'
        (feature_def derived var 'relatedFeature' : 'Feature' multiplicity :>> 'relatedElement' ordered nonunique)
        (feature_def derived var 'association' : 'Association' multiplicity :>> ''type'' ordered)
        (feature_def derived var 'connectorEnd' : 'Feature' multiplicity :>> 'endFeature' ordered)
        (feature_def derived var 'sourceFeature' : 'Feature' multiplicity :> 'relatedFeature' :>> 'source' ordered)
        (feature_def derived var 'targetFeature' : 'Feature' multiplicity :> 'relatedFeature' :>> 'target' ordered)
        (feature_def derived var 'defaultFeaturingType' : 'Type' multiplicity))
      (metaclass_def 'ConstructorExpression' :> 'InstantiationExpression')
      (metaclass_def 'DataType' :> 'Classifier')
      (metaclass_def 'ElementFilterMembership' :> 'OwningMembership'
        (feature_def derived composite var 'condition' : 'Expression' multiplicity :>> 'ownedMemberElement'))
      (metaclass_def 'Expression' :> 'Step'
        (feature_def derived var 'isModelLevelEvaluable' : 'Boolean' multiplicity)
        (feature_def derived var ''function'' : 'Function' multiplicity :>> ''behavior'')
        (feature_def derived var 'result' : 'Feature' multiplicity :> 'output', 'parameter'))
      (metaclass_def 'FeatureChainExpression' :> 'OperatorExpression'
        (feature_def var 'operator' : 'String' multiplicity :>> 'operator')
        (feature_def derived var 'targetFeature' : 'Feature' multiplicity :> ''member''))
      (metaclass_def 'FeatureReferenceExpression' :> 'Expression'
        (feature_def derived var 'referent' : 'Feature' multiplicity :> ''member''))
      (metaclass_def 'FeatureValue' :> 'OwningMembership'
        (feature_def var 'isInitial' : 'Boolean' multiplicity)
        (feature_def var 'isDefault' : 'Boolean' multiplicity)
        (feature_def derived var 'featureWithValue' : 'Feature' multiplicity :> 'membershipOwningNamespace')
        (feature_def derived composite var 'value' : 'Expression' multiplicity :>> 'ownedMemberElement'))
      (metaclass_def 'Flow' :> 'Connector', 'Step'
        (feature_def derived var 'payloadType' : 'Classifier' multiplicity ordered nonunique)
        (feature_def derived var 'targetInputFeature' : 'Feature' multiplicity ordered nonunique)
        (feature_def derived var 'sourceOutputFeature' : 'Feature' multiplicity ordered nonunique)
        (feature_def derived var 'flowEnd' : 'FlowEnd' multiplicity :> 'connectorEnd' ordered)
        (feature_def derived var 'payloadFeature' : 'PayloadFeature' multiplicity :> 'ownedFeature')
        (feature_def derived var ''interaction'' : 'Interaction' multiplicity :>> 'association', ''behavior'' ordered))
      (metaclass_def 'FlowEnd' :> 'Feature')
      (metaclass_def 'Function' :> 'Behavior'
        (feature_def derived var 'isModelLevelEvaluable' : 'Boolean' multiplicity)
        (feature_def derived var 'expression' : 'Expression' multiplicity :> ''step'')
        (feature_def derived var 'result' : 'Feature' multiplicity :> 'output', 'parameter'))
      (metaclass_def 'IndexExpression' :> 'OperatorExpression'
        (feature_def var 'operator' : 'String' multiplicity :>> 'operator'))
      (metaclass_def abstract 'InstantiationExpression' :> 'Expression'
        (feature_def derived var 'argument' : 'Expression' multiplicity ordered)
        (feature_def derived var 'instantiatedType' : 'Type' multiplicity :> ''member''))
      (metaclass_def 'Interaction' :> 'Association', 'Behavior')
      (metaclass_def 'Invariant' :> 'BooleanExpression'
        (feature_def var 'isNegated' : 'Boolean' multiplicity))
      (metaclass_def 'InvocationExpression' :> 'InstantiationExpression')
      (metaclass_def 'LibraryPackage' :> 'Package'
        (feature_def var 'isStandard' : 'Boolean' multiplicity))
      (metaclass_def 'LiteralBoolean' :> 'LiteralExpression'
        (feature_def var 'value' : 'Boolean' multiplicity))
      (metaclass_def 'LiteralExpression' :> 'Expression')
      (metaclass_def 'LiteralInfinity' :> 'LiteralExpression')
      (metaclass_def 'LiteralInteger' :> 'LiteralExpression'
        (feature_def var 'value' : 'Integer' multiplicity))
      (metaclass_def 'LiteralRational' :> 'LiteralExpression'
        (feature_def var 'value' : 'Rational' multiplicity))
      (metaclass_def 'LiteralString' :> 'LiteralExpression'
        (feature_def var 'value' : 'String' multiplicity))
      (metaclass_def 'Metaclass' :> 'Structure')
      (metaclass_def 'MetadataAccessExpression' :> 'Expression'
        (feature_def derived var 'referencedElement' : 'Element' multiplicity :> ''member''))
      (metaclass_def 'MetadataFeature' :> 'AnnotatingElement', 'Feature'
        (feature_def derived var ''metaclass'' : 'Metaclass' multiplicity :> ''type''))
      (metaclass_def 'MultiplicityRange' :> 'Multiplicity'
        (feature_def derived var 'lowerBound' : 'Expression' multiplicity :> 'bound')
        (feature_def derived var 'upperBound' : 'Expression' multiplicity :> 'bound')
        (feature_def derived var 'bound' : 'Expression' multiplicity :> 'ownedMember' ordered))
      (metaclass_def 'NullExpression' :> 'Expression')
      (metaclass_def 'OperatorExpression' :> 'InvocationExpression'
        (feature_def var 'operator' : 'String' multiplicity))
      (metaclass_def 'Package' :> 'Namespace'
        (feature_def derived var 'filterCondition' : 'Expression' multiplicity :> 'ownedMember' ordered))
      (metaclass_def 'ParameterMembership' :> 'FeatureMembership'
        (feature_def derived composite var 'ownedMemberParameter' : 'Feature' multiplicity :>> 'ownedMemberFeature'))
      (metaclass_def 'PayloadFeature' :> 'Feature')
      (metaclass_def 'Predicate' :> 'Function')
      (metaclass_def 'ResultExpressionMembership' :> 'FeatureMembership'
        (feature_def derived composite var 'ownedResultExpression' : 'Expression' multiplicity :>> 'ownedMemberFeature'))
      (metaclass_def 'ReturnParameterMembership' :> 'ParameterMembership')
      (metaclass_def 'SelectExpression' :> 'OperatorExpression'
        (feature_def var 'operator' : 'String' multiplicity :>> 'operator'))
      (metaclass_def 'Step' :> 'Feature'
        (feature_def derived var ''behavior'' : 'Behavior' multiplicity :> ''type'' ordered)
        (feature_def derived var 'parameter' : 'Feature' multiplicity :>> 'directedFeature' ordered))
      (metaclass_def 'Structure' :> 'Class')
      (metaclass_def 'Succession' :> 'Connector')
      (metaclass_def 'SuccessionFlow' :> 'Succession', 'Flow'))))
~~~
# FORMAT
~~~sysml
standard library package KerML {
    doc /*
	 * This package contains a reflective KerML model of the KerML abstract syntax.
	 */

    private import ScalarValues::*;
    public import Kernel::*;

    package Root {
        metaclass AnnotatingElement specializes Element {
            derived var feature annotatedElement : Element [1..*] redefines annotatedElement ordered;
            derived composite var feature ownedAnnotatingRelationship : Annotation [0..*] subsets annotation, ownedRelationship ordered;
            derived var feature owningAnnotatingRelationship : Annotation [0..1] subsets owningRelationship, annotation;
            derived var feature annotation : Annotation [0..*] ordered;
        }

        metaclass Annotation specializes Relationship {
            var feature annotatedElement : Element [1..1] redefines target, annotatedElement;
            derived var feature annotatingElement : AnnotatingElement [1..1] redefines source;
            derived var feature owningAnnotatedElement : Element [0..1] subsets annotatedElement, owningRelatedElement;
            derived var feature owningAnnotatingElement : AnnotatingElement [0..1] subsets annotatingElement, owningRelatedElement;
            derived composite var feature ownedAnnotatingElement : AnnotatingElement [0..1] subsets annotatingElement, ownedRelatedElement;
        }

        metaclass Comment specializes AnnotatingElement {
            var feature 'locale' : String [0..1];
            var feature body : String [1..1];
        }

        metaclass Dependency specializes Relationship {
            var feature client : Element [1..*] redefines source ordered;
            var feature supplier : Element [1..*] redefines target ordered;
        }

        metaclass Documentation specializes Comment {
            derived var feature documentedElement : Element [1..1] subsets owner redefines annotatedElement;
        }

        abstract metaclass Element {
            var feature elementId : String [1..1];
            var feature aliasIds : String [0..*] ordered;
            var feature declaredShortName : String [0..1];
            var feature declaredName : String [0..1];
            var feature isImpliedIncluded : Boolean [1..1];
            derived var feature shortName : String [0..1];
            derived var feature name : String [0..1];
            derived var feature qualifiedName : String [0..1];
            derived var feature isLibraryElement : Boolean [1..1];

            var feature owningRelationship : Relationship [0..1];
            composite var feature ownedRelationship : Relationship [0..*] ordered;
            derived var feature owningMembership : OwningMembership [0..1] subsets owningRelationship;
            derived var feature owningNamespace : Namespace [0..1];
            derived var feature owner : Element [0..1];
            derived var feature ownedElement : Element [0..*] ordered;
            derived var feature documentation : Documentation [0..*] subsets ownedElement ordered;
            derived composite var feature ownedAnnotation : Annotation [0..*] subsets ownedRelationship ordered;
            derived var feature textualRepresentation : TextualRepresentation [0..*] subsets ownedElement ordered;
        }

        abstract metaclass Import specializes Relationship {
            var feature visibility : VisibilityKind [1..1];
            var feature isRecursive : Boolean [1..1];
            var feature isImportAll : Boolean [1..1];

            derived var feature importOwningNamespace : Namespace [1..1] subsets owningRelatedElement redefines source;
            derived var feature importedElement : Element [1..1];
        }

        metaclass Membership specializes Relationship {
            var feature memberShortName : String [0..1];
            var feature memberName : String [0..1];
            var feature visibility : VisibilityKind [1..1];
            derived var feature memberElementId : String [1..1];

            var feature memberElement : Element [1..1] redefines target;
            derived var feature membershipOwningNamespace : Namespace [1..1] subsets owningRelatedElement redefines source;
        }

        metaclass MembershipImport specializes Import {
            var feature importedMembership : Membership [1..1] redefines target;
        }

        metaclass Namespace specializes Element {
            derived abstract var feature membership : Membership [0..*] ordered;
            derived composite var feature ownedImport : Import [0..*] subsets ownedRelationship ordered;
            derived var feature 'member' : Element [0..*] ordered;
            derived var feature ownedMember : Element [0..*] subsets 'member' ordered;
            derived composite var feature ownedMembership : Membership [0..*] subsets membership, ownedRelationship ordered;
            derived var feature importedMembership : Membership [0..*] subsets membership ordered;
        }

        metaclass NamespaceImport specializes Import {
            var feature importedNamespace : Namespace [1..1] redefines target;
        }

        metaclass OwningMembership specializes Membership {
            derived var feature ownedMemberElementId : String [1..1] redefines memberElementId;
            derived var feature ownedMemberShortName : String [0..1] redefines memberShortName;
            derived var feature ownedMemberName : String [0..1] redefines memberName;

            derived composite var feature ownedMemberElement : Element [1..1] subsets ownedRelatedElement redefines memberElement;
        }

        abstract metaclass Relationship specializes Element {
            var feature isImplied : Boolean [1..1];

            var feature target : Element [0..*] subsets relatedElement ordered;
            var feature source : Element [0..*] subsets relatedElement ordered;
            var feature owningRelatedElement : Element [0..1] subsets relatedElement;
            composite var feature ownedRelatedElement : Element [0..*] subsets relatedElement ordered;
            derived var feature relatedElement : Element [0..*] ordered nonunique;
        }

        metaclass TextualRepresentation specializes AnnotatingElement {
            var feature 'language' : String [1..1];
            var feature body : String [1..1];

            derived var feature representedElement : Element [1..1] subsets owner redefines annotatedElement;
        }

        datatype VisibilityKind {
            member feature 'private' : VisibilityKind [1];
            member feature 'protected' : VisibilityKind [1];
            member feature 'public' : VisibilityKind [1];
        }
    }

    package Core {
        public import Root::*;

        metaclass Classifier specializes Type {
            derived composite var feature ownedSubclassification : Subclassification [0..*] subsets ownedSpecialization;
        }

        metaclass Conjugation specializes Relationship {
            var feature originalType : Type [1..1] redefines target;
            var feature conjugatedType : Type [1..1] redefines source;
            derived var feature owningType : Type [0..1] subsets conjugatedType, owningRelatedElement;
        }

        metaclass CrossSubsetting specializes Subsetting {
            var feature crossedFeature : Feature [1..1] redefines subsettedFeature;
            derived var feature crossingFeature : Feature [1..1] redefines owningFeature, subsettingFeature;
        }

        metaclass Differencing specializes Relationship {
            var feature differencingType : Type [1..1] redefines target;
            derived var feature typeDifferenced : Type [1..1] subsets owningRelatedElement redefines source;
        }

        metaclass Disjoining specializes Relationship {
            var feature typeDisjoined : Type [1..1] redefines source;
            var feature disjoiningType : Type [1..1] redefines target;
            derived var feature owningType : Type [0..1] subsets owningRelatedElement, typeDisjoined;
        }

        metaclass EndFeatureMembership specializes FeatureMembership {
            derived composite var feature ownedMemberFeature : Feature [1..1] redefines ownedMemberFeature;
        }

        metaclass Feature specializes Type {
            var feature isUnique : Boolean [1..1];
            var feature isOrdered : Boolean [1..1];
            var feature isComposite : Boolean [1..1];
            var feature isEnd : Boolean [1..1];
            var feature isDerived : Boolean [1..1];
            var feature isPortion : Boolean [1..1];
            var feature isVariable : Boolean [1..1];
            var feature isConstant : Boolean [1..1];
            var feature direction : FeatureDirectionKind [0..1];

            derived var feature owningType : Type [0..1] subsets owningNamespace, featuringType;
            derived var feature 'type' : Type [0..*] ordered;
            derived composite var feature ownedRedefinition : Redefinition [0..*] subsets ownedSubsetting;
            derived composite var feature ownedSubsetting : Subsetting [0..*] subsets ownedSpecialization;
            derived var feature owningFeatureMembership : FeatureMembership [0..1] subsets owningMembership;
            derived var feature endOwningType : Type [0..1] subsets owningType;
            derived composite var feature ownedTyping : FeatureTyping [0..*] subsets ownedSpecialization ordered;
            derived var feature featuringType : Type [0..*] ordered;
            derived composite var feature ownedTypeFeaturing : TypeFeaturing [0..*] subsets ownedRelationship ordered;
            derived var feature chainingFeature : Feature [0..*] ordered nonunique;
            derived composite var feature ownedFeatureInverting : FeatureInverting [0..*] subsets ownedRelationship;
            derived composite var feature ownedFeatureChaining : FeatureChaining [0..*] subsets ownedRelationship ordered;
            derived composite var feature ownedReferenceSubsetting : ReferenceSubsetting [0..1] subsets ownedSubsetting;
            derived var feature featureTarget : Feature [1..1];
            derived var feature crossFeature : Feature [0..1];
            derived composite var feature ownedCrossSubsetting : CrossSubsetting [0..1] subsets ownedSubsetting;
        }

        metaclass FeatureChaining specializes Relationship {
            var feature chainingFeature : Feature [1..1] redefines target;
            derived var feature featureChained : Feature [1..1] subsets owningRelatedElement redefines source;
        }

        datatype FeatureDirectionKind {
            member feature 'in' : FeatureDirectionKind [1];
            member feature 'inout' : FeatureDirectionKind [1];
            member feature 'out' : FeatureDirectionKind [1];
        }

        metaclass FeatureInverting specializes Relationship {
            var feature featureInverted : Feature [1..1] redefines source;
            var feature invertingFeature : Feature [1..1] redefines target;
            derived var feature owningFeature : Feature [0..1] subsets featureInverted, owningRelatedElement;
        }

        metaclass FeatureMembership specializes OwningMembership {
            derived var feature owningType : Type [1..1] redefines membershipOwningNamespace;
            derived composite var feature ownedMemberFeature : Feature [1..1] redefines ownedMemberElement;
        }

        metaclass FeatureTyping specializes Specialization {
            var feature typedFeature : Feature [1..1] redefines specific;
            var feature 'type' : Type [1..1] redefines general;
            derived var feature owningFeature : Feature [0..1] subsets typedFeature redefines owningType;
        }

        metaclass Intersecting specializes Relationship {
            var feature intersectingType : Type [1..1] redefines target;
            derived var feature typeIntersected : Type [1..1] subsets owningRelatedElement redefines source;
        }

        metaclass Multiplicity specializes Feature;

        metaclass Redefinition specializes Subsetting {
            var feature redefiningFeature : Feature [1..1] redefines subsettingFeature;
            var feature redefinedFeature : Feature [1..1] redefines subsettedFeature;
        }

        metaclass ReferenceSubsetting specializes Subsetting {
            var feature referencedFeature : Feature [1..1] redefines subsettedFeature;
            derived var feature referencingFeature : Feature [1..1] redefines owningFeature, subsettingFeature;
        }

        metaclass Specialization specializes Relationship {
            var feature general : Type [1..1] redefines target;
            var feature specific : Type [1..1] redefines source;
            derived var feature owningType : Type [0..1] subsets owningRelatedElement, specific;
        }

        metaclass Subclassification specializes Specialization {
            var feature superclassifier : Classifier [1..1] redefines general;
            var feature 'subclassifier' : Classifier [1..1] redefines specific;
            derived var feature owningClassifier : Classifier [0..1] redefines owningType;
        }

        metaclass Subsetting specializes Specialization {
            var feature subsettedFeature : Feature [1..1] redefines general;
            var feature subsettingFeature : Feature [1..1] redefines specific;
            derived var feature owningFeature : Feature [0..1] subsets subsettingFeature redefines owningType;
        }

        metaclass Type specializes Namespace {
            var feature isAbstract : Boolean [1..1];
            var feature isSufficient : Boolean [1..1];
            derived var feature isConjugated : Boolean [1..1];

            derived composite var feature ownedSpecialization : Specialization [0..*] subsets ownedRelationship ordered;
            derived composite var feature ownedFeatureMembership : FeatureMembership [0..*] subsets ownedMembership, featureMembership ordered;
            derived var feature 'feature' : Feature [0..*] subsets 'member' ordered;
            derived var feature ownedFeature : Feature [0..*] subsets ownedMember ordered;
            derived var feature input : Feature [0..*] subsets directedFeature ordered;
            derived var feature output : Feature [0..*] subsets directedFeature ordered;
            derived var feature inheritedMembership : Membership [0..*] subsets membership ordered;
            derived var feature endFeature : Feature [0..*] subsets 'feature' ordered;
            derived var feature ownedEndFeature : Feature [0..*] subsets endFeature, ownedFeature ordered;
            derived composite var feature ownedConjugator : Conjugation [0..1] subsets ownedRelationship;
            derived var feature inheritedFeature : Feature [0..*] subsets 'feature' ordered;
            derived var feature 'multiplicity' : Multiplicity [0..1] subsets ownedMember;
            derived var feature unioningType : Type [0..*] ordered;
            derived composite var feature ownedIntersecting : Intersecting [0..*] subsets ownedRelationship ordered;
            derived var feature intersectingType : Type [0..*] ordered;
            derived composite var feature ownedUnioning : Unioning [0..*] subsets ownedRelationship ordered;
            derived composite var feature ownedDisjoining : Disjoining [0..*] subsets ownedRelationship;
            derived var feature featureMembership : FeatureMembership [0..*] ordered;
            derived var feature differencingType : Type [0..*] ordered;
            derived composite var feature ownedDifferencing : Differencing [0..*] subsets ownedRelationship ordered;
            derived var feature directedFeature : Feature [0..*] subsets 'feature' ordered;
        }

        metaclass TypeFeaturing specializes Relationship {
            var feature featureOfType : Feature [1..1] redefines source;
            var feature featuringType : Type [1..1] redefines target;
            derived var feature owningFeatureOfType : Feature [0..1] subsets owningRelatedElement, featureOfType;
        }

        metaclass Unioning specializes Relationship {
            var feature unioningType : Type [1..1] redefines target;
            derived var feature typeUnioned : Type [1..1] subsets owningRelatedElement redefines source;
        }
    }

    package Kernel {
        public import Core::*;

        metaclass Association specializes Classifier, Relationship {
            derived var feature relatedType : Type [0..*] redefines relatedElement ordered nonunique;
            derived var feature sourceType : Type [0..1] subsets relatedType redefines source;
            derived var feature targetType : Type [0..*] subsets relatedType redefines target;
            derived var feature associationEnd : Feature [0..*] redefines endFeature;
        }

        metaclass AssociationStructure specializes Association, Structure;

        metaclass Behavior specializes Class {
            derived var feature 'step' : Step [0..*] subsets 'feature';
            derived var feature parameter : Feature [0..*] redefines directedFeature ordered;
        }

        metaclass BindingConnector specializes Connector;

        metaclass BooleanExpression specializes Expression {
            derived var feature 'predicate' : Predicate [0..1] redefines 'function';
        }

        metaclass Class specializes Classifier;

        metaclass CollectExpression specializes OperatorExpression {
            var feature operator : String [1..1] redefines operator;
        }

        metaclass Connector specializes Feature, Relationship {
            derived var feature relatedFeature : Feature [0..*] redefines relatedElement ordered nonunique;
            derived var feature association : Association [0..*] redefines 'type' ordered;
            derived var feature connectorEnd : Feature [0..*] redefines endFeature ordered;
            derived var feature sourceFeature : Feature [0..1] subsets relatedFeature redefines source ordered;
            derived var feature targetFeature : Feature [0..*] subsets relatedFeature redefines target ordered;
            derived var feature defaultFeaturingType : Type [0..1];
        }

        metaclass ConstructorExpression specializes InstantiationExpression;

        metaclass DataType specializes Classifier;

        metaclass ElementFilterMembership specializes OwningMembership {
            derived composite var feature condition : Expression [1..1] redefines ownedMemberElement;
        }

        metaclass Expression specializes Step {
            derived var feature isModelLevelEvaluable : Boolean [1..1];

            derived var feature 'function' : Function [0..1] redefines 'behavior';
            derived var feature result : Feature [1..1] subsets output, parameter;
        }

        metaclass FeatureChainExpression specializes OperatorExpression {
            var feature operator : String [1..1] redefines operator;

            derived var feature targetFeature : Feature [1..1] subsets 'member';
        }

        metaclass FeatureReferenceExpression specializes Expression {
            derived var feature referent : Feature [1..1] subsets 'member';
        }

        metaclass FeatureValue specializes OwningMembership {
            var feature isInitial : Boolean [1..1];
            var feature isDefault : Boolean [1..1];

            derived var feature featureWithValue : Feature [1..1] subsets membershipOwningNamespace;
            derived composite var feature value : Expression [1..1] redefines ownedMemberElement;
        }

        metaclass Flow specializes Connector, Step {
            derived var feature payloadType : Classifier [0..*] ordered nonunique;
            derived var feature targetInputFeature : Feature [0..1] ordered nonunique;
            derived var feature sourceOutputFeature : Feature [0..1] ordered nonunique;
            derived var feature flowEnd : FlowEnd [0..2] subsets connectorEnd ordered;
            derived var feature payloadFeature : PayloadFeature [0..1] subsets ownedFeature;
            derived var feature 'interaction' : Interaction [0..*] redefines association, 'behavior' ordered;
        }

        metaclass FlowEnd specializes Feature;

        metaclass Function specializes Behavior {
            derived var feature isModelLevelEvaluable : Boolean [1..1];

            derived var feature expression : Expression [0..*] subsets 'step';
            derived var feature result : Feature [1..1] subsets output, parameter;
        }

        metaclass IndexExpression specializes OperatorExpression {
            var feature operator : String [1..1] redefines operator;
        }

        abstract metaclass InstantiationExpression specializes Expression {
            derived var feature argument : Expression [0..*] ordered;
            derived var feature instantiatedType : Type [1..1] subsets 'member';
        }

        metaclass Interaction specializes Association, Behavior;

        metaclass Invariant specializes BooleanExpression {
            var feature isNegated : Boolean [1..1];
        }

        metaclass InvocationExpression specializes InstantiationExpression;

        metaclass LibraryPackage specializes Package {
            var feature isStandard : Boolean [1..1];
        }

        metaclass LiteralBoolean specializes LiteralExpression {
            var feature value : Boolean [1..1];
        }

        metaclass LiteralExpression specializes Expression;

        metaclass LiteralInfinity specializes LiteralExpression;

        metaclass LiteralInteger specializes LiteralExpression {
            var feature value : Integer [1..1];
        }

        metaclass LiteralRational specializes LiteralExpression {
            var feature value : Rational [1..1];
        }

        metaclass LiteralString specializes LiteralExpression {
            var feature value : String [1..1];
        }

        metaclass Metaclass specializes Structure;

        metaclass MetadataAccessExpression specializes Expression {
            derived var feature referencedElement : Element [1..1] subsets 'member';
        }

        metaclass MetadataFeature specializes AnnotatingElement, Feature {
            derived var feature 'metaclass' : Metaclass [0..1] subsets 'type';
        }

        metaclass MultiplicityRange specializes Multiplicity {
            derived var feature lowerBound : Expression [0..1] subsets bound;
            derived var feature upperBound : Expression [1..1] subsets bound;
            derived var feature bound : Expression [1..2] subsets ownedMember ordered;
        }

        metaclass NullExpression specializes Expression;

        metaclass OperatorExpression specializes InvocationExpression {
            var feature operator : String [1..1];
        }

        metaclass Package specializes Namespace {
            derived var feature filterCondition : Expression [0..*] subsets ownedMember ordered;
        }

        metaclass ParameterMembership specializes FeatureMembership {
            derived composite var feature ownedMemberParameter : Feature [1..1] redefines ownedMemberFeature;
        }

        metaclass PayloadFeature specializes Feature;

        metaclass Predicate specializes Function;

        metaclass ResultExpressionMembership specializes FeatureMembership {
            derived composite var feature ownedResultExpression : Expression [1..1] redefines ownedMemberFeature;
        }

        metaclass ReturnParameterMembership specializes ParameterMembership;

        metaclass SelectExpression specializes OperatorExpression {
            var feature operator : String [1..1] redefines operator;
        }

        metaclass Step specializes Feature {
            derived var feature 'behavior' : Behavior [0..*] subsets 'type' ordered;
            derived var feature parameter : Feature [0..*] redefines directedFeature ordered;
        }

        metaclass Structure specializes Class;

        metaclass Succession specializes Connector;

        metaclass SuccessionFlow specializes Succession, Flow;
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "KerML"))) (name "KerML") (declared-name "KerML")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "KerML::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "KerML::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "KerML::Core"))) (name "Core") (declared-name "Core")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "KerML::Core::*"))) (name "*") (declared-name "*"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Classifier"))) (name "Classifier") (declared-name "Classifier"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Conjugation"))) (name "Conjugation") (declared-name "Conjugation"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::CrossSubsetting"))) (name "CrossSubsetting") (declared-name "CrossSubsetting"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Differencing"))) (name "Differencing") (declared-name "Differencing"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Disjoining"))) (name "Disjoining") (declared-name "Disjoining"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::EndFeatureMembership"))) (name "EndFeatureMembership") (declared-name "EndFeatureMembership"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Feature"))) (name "Feature") (declared-name "Feature"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::FeatureChaining"))) (name "FeatureChaining") (declared-name "FeatureChaining"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::FeatureDirectionKind"))) (name "FeatureDirectionKind") (declared-name "FeatureDirectionKind"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::FeatureInverting"))) (name "FeatureInverting") (declared-name "FeatureInverting"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::FeatureMembership"))) (name "FeatureMembership") (declared-name "FeatureMembership"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::FeatureTyping"))) (name "FeatureTyping") (declared-name "FeatureTyping"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Intersecting"))) (name "Intersecting") (declared-name "Intersecting"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Multiplicity"))) (name "Multiplicity") (declared-name "Multiplicity"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Redefinition"))) (name "Redefinition") (declared-name "Redefinition"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::ReferenceSubsetting"))) (name "ReferenceSubsetting") (declared-name "ReferenceSubsetting"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Specialization"))) (name "Specialization") (declared-name "Specialization"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Subclassification"))) (name "Subclassification") (declared-name "Subclassification"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Subsetting"))) (name "Subsetting") (declared-name "Subsetting"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Type"))) (name "Type") (declared-name "Type"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::TypeFeaturing"))) (name "TypeFeaturing") (declared-name "TypeFeaturing"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Core::Unioning"))) (name "Unioning") (declared-name "Unioning"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "KerML::Kernel"))) (name "Kernel") (declared-name "Kernel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "KerML::Kernel::*"))) (name "*") (declared-name "*"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Association"))) (name "Association") (declared-name "Association"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::AssociationStructure"))) (name "AssociationStructure") (declared-name "AssociationStructure"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Behavior"))) (name "Behavior") (declared-name "Behavior"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::BindingConnector"))) (name "BindingConnector") (declared-name "BindingConnector"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::BooleanExpression"))) (name "BooleanExpression") (declared-name "BooleanExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Class"))) (name "Class") (declared-name "Class"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::CollectExpression"))) (name "CollectExpression") (declared-name "CollectExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Connector"))) (name "Connector") (declared-name "Connector"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::ConstructorExpression"))) (name "ConstructorExpression") (declared-name "ConstructorExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::DataType"))) (name "DataType") (declared-name "DataType"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::ElementFilterMembership"))) (name "ElementFilterMembership") (declared-name "ElementFilterMembership"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Expression"))) (name "Expression") (declared-name "Expression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::FeatureChainExpression"))) (name "FeatureChainExpression") (declared-name "FeatureChainExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::FeatureReferenceExpression"))) (name "FeatureReferenceExpression") (declared-name "FeatureReferenceExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::FeatureValue"))) (name "FeatureValue") (declared-name "FeatureValue"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Flow"))) (name "Flow") (declared-name "Flow"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::FlowEnd"))) (name "FlowEnd") (declared-name "FlowEnd"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Function"))) (name "Function") (declared-name "Function"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::IndexExpression"))) (name "IndexExpression") (declared-name "IndexExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::InstantiationExpression"))) (name "InstantiationExpression") (declared-name "InstantiationExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Interaction"))) (name "Interaction") (declared-name "Interaction"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Invariant"))) (name "Invariant") (declared-name "Invariant"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::InvocationExpression"))) (name "InvocationExpression") (declared-name "InvocationExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::LibraryPackage"))) (name "LibraryPackage") (declared-name "LibraryPackage"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralBoolean"))) (name "LiteralBoolean") (declared-name "LiteralBoolean"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralExpression"))) (name "LiteralExpression") (declared-name "LiteralExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralInfinity"))) (name "LiteralInfinity") (declared-name "LiteralInfinity"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralInteger"))) (name "LiteralInteger") (declared-name "LiteralInteger"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralRational"))) (name "LiteralRational") (declared-name "LiteralRational"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralString"))) (name "LiteralString") (declared-name "LiteralString"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Metaclass"))) (name "Metaclass") (declared-name "Metaclass"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::MetadataAccessExpression"))) (name "MetadataAccessExpression") (declared-name "MetadataAccessExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::MetadataFeature"))) (name "MetadataFeature") (declared-name "MetadataFeature"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::MultiplicityRange"))) (name "MultiplicityRange") (declared-name "MultiplicityRange"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::NullExpression"))) (name "NullExpression") (declared-name "NullExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::OperatorExpression"))) (name "OperatorExpression") (declared-name "OperatorExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Package"))) (name "Package") (declared-name "Package"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::ParameterMembership"))) (name "ParameterMembership") (declared-name "ParameterMembership"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::PayloadFeature"))) (name "PayloadFeature") (declared-name "PayloadFeature"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Predicate"))) (name "Predicate") (declared-name "Predicate"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::ResultExpressionMembership"))) (name "ResultExpressionMembership") (declared-name "ResultExpressionMembership"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::ReturnParameterMembership"))) (name "ReturnParameterMembership") (declared-name "ReturnParameterMembership"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::SelectExpression"))) (name "SelectExpression") (declared-name "SelectExpression"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Step"))) (name "Step") (declared-name "Step"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Structure"))) (name "Structure") (declared-name "Structure"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::Succession"))) (name "Succession") (declared-name "Succession"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Kernel::SuccessionFlow"))) (name "SuccessionFlow") (declared-name "SuccessionFlow"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "KerML::Root"))) (name "Root") (declared-name "Root")
          (contains
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::AnnotatingElement"))) (name "AnnotatingElement") (declared-name "AnnotatingElement"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Annotation"))) (name "Annotation") (declared-name "Annotation"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Comment"))) (name "Comment") (declared-name "Comment"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Dependency"))) (name "Dependency") (declared-name "Dependency"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Documentation"))) (name "Documentation") (declared-name "Documentation"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Element"))) (name "Element") (declared-name "Element"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Import"))) (name "Import") (declared-name "Import"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Membership"))) (name "Membership") (declared-name "Membership"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::MembershipImport"))) (name "MembershipImport") (declared-name "MembershipImport"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Namespace"))) (name "Namespace") (declared-name "Namespace"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::NamespaceImport"))) (name "NamespaceImport") (declared-name "NamespaceImport"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::OwningMembership"))) (name "OwningMembership") (declared-name "OwningMembership"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::Relationship"))) (name "Relationship") (declared-name "Relationship"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::TextualRepresentation"))) (name "TextualRepresentation") (declared-name "TextualRepresentation"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KerML::Root::VisibilityKind"))) (name "VisibilityKind") (declared-name "VisibilityKind"))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "KerML::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "KerML::_documentation"))) (to (node (document "d0") (qualified-name "KerML"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
