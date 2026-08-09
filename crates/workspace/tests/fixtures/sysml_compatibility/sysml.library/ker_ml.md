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
(model
  (namespace
    (library_package 'KerML'
      (documentation)
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import public -> 'KerML::Kernel'[package])
      (package 'Root'
        (metaclass_def 'AnnotatingElement' :> 'KerML::Root::Element'[metaclass_def]
          (feature_def derived ordered 'annotatedElement' : 'KerML::Root::Element'[metaclass_def] :>> 'annotatedElement'[unresolved]
            (multiplicity_range [1..*]))
          (feature_def derived composite ordered 'ownedAnnotatingRelationship' : 'KerML::Root::Annotation'[metaclass_def] :> 'KerML::Root::AnnotatingElement::annotation'[feature_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'owningAnnotatingRelationship' : 'KerML::Root::Annotation'[metaclass_def] :> 'KerML::Root::Element::owningRelationship'[feature_def] :> 'KerML::Root::AnnotatingElement::annotation'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'annotation' : 'KerML::Root::Annotation'[metaclass_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'Annotation' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'annotatedElement' : 'KerML::Root::Element'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def] :>> 'annotatedElement'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived 'annotatingElement' : 'KerML::Root::AnnotatingElement'[metaclass_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningAnnotatedElement' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Annotation::annotatedElement'[feature_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'owningAnnotatingElement' : 'KerML::Root::AnnotatingElement'[metaclass_def] :> 'KerML::Root::Annotation::annotatingElement'[feature_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived composite 'ownedAnnotatingElement' : 'KerML::Root::AnnotatingElement'[metaclass_def] :> 'KerML::Root::Annotation::annotatingElement'[feature_def] :> 'KerML::Root::Relationship::ownedRelatedElement'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'Comment' :> 'KerML::Root::AnnotatingElement'[metaclass_def]
          (feature_def 'locale' : 'String'[unresolved]
            (multiplicity_range [0..1]))
          (feature_def 'body' : 'String'[unresolved]
            (multiplicity_range [1..1])))
        (metaclass_def 'Dependency' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def ordered 'client' : 'KerML::Root::Element'[metaclass_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..*]))
          (feature_def ordered 'supplier' : 'KerML::Root::Element'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..*])))
        (metaclass_def 'Documentation' :> 'KerML::Root::Comment'[metaclass_def]
          (feature_def derived 'documentedElement' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Element::owner'[feature_def] :>> 'KerML::Root::AnnotatingElement::annotatedElement'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def abstract 'Element'
          (feature_def 'elementId' : 'String'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def ordered 'aliasIds' : 'String'[unresolved]
            (multiplicity_range [0..*]))
          (feature_def 'declaredShortName' : 'String'[unresolved]
            (multiplicity_range [0..1]))
          (feature_def 'declaredName' : 'String'[unresolved]
            (multiplicity_range [0..1]))
          (feature_def 'isImpliedIncluded' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived 'shortName' : 'String'[unresolved]
            (multiplicity_range [0..1]))
          (feature_def derived 'name' : 'String'[unresolved]
            (multiplicity_range [0..1]))
          (feature_def derived 'qualifiedName' : 'String'[unresolved]
            (multiplicity_range [0..1]))
          (feature_def derived 'isLibraryElement' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'owningRelationship' : 'KerML::Root::Relationship'[metaclass_def]
            (multiplicity_range [0..1]))
          (feature_def composite ordered 'ownedRelationship' : 'KerML::Root::Relationship'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'owningMembership' : 'KerML::Root::OwningMembership'[metaclass_def] :> 'KerML::Root::Element::owningRelationship'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'owningNamespace' : 'KerML::Root::Namespace'[metaclass_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'owner' : 'KerML::Root::Element'[metaclass_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'ownedElement' : 'KerML::Root::Element'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'documentation' : 'KerML::Root::Documentation'[metaclass_def] :> 'KerML::Root::Element::ownedElement'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedAnnotation' : 'KerML::Root::Annotation'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'textualRepresentation' : 'KerML::Root::TextualRepresentation'[metaclass_def] :> 'KerML::Root::Element::ownedElement'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def abstract 'Import' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'visibility' : 'KerML::Root::VisibilityKind'[datatype_def]
            (multiplicity_range [1..1]))
          (feature_def 'isRecursive' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isImportAll' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived 'importOwningNamespace' : 'KerML::Root::Namespace'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'importedElement' : 'KerML::Root::Element'[metaclass_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Membership' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'memberShortName' : 'String'[unresolved]
            (multiplicity_range [0..1]))
          (feature_def 'memberName' : 'String'[unresolved]
            (multiplicity_range [0..1]))
          (feature_def 'visibility' : 'KerML::Root::VisibilityKind'[datatype_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'memberElementId' : 'String'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'memberElement' : 'KerML::Root::Element'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'membershipOwningNamespace' : 'KerML::Root::Namespace'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'MembershipImport' :> 'KerML::Root::Import'[metaclass_def]
          (feature_def 'importedMembership' : 'KerML::Root::Membership'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Namespace' :> 'KerML::Root::Element'[metaclass_def]
          (feature_def abstract derived ordered 'membership' : 'KerML::Root::Membership'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedImport' : 'KerML::Root::Import'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'member' : 'KerML::Root::Element'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'ownedMember' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Namespace::member'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedMembership' : 'KerML::Root::Membership'[metaclass_def] :> 'KerML::Root::Namespace::membership'[feature_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'importedMembership' : 'KerML::Root::Membership'[metaclass_def] :> 'KerML::Root::Namespace::membership'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'NamespaceImport' :> 'KerML::Root::Import'[metaclass_def]
          (feature_def 'importedNamespace' : 'KerML::Root::Namespace'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'OwningMembership' :> 'KerML::Root::Membership'[metaclass_def]
          (feature_def derived 'ownedMemberElementId' : 'String'[unresolved] :>> 'KerML::Root::Membership::memberElementId'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'ownedMemberShortName' : 'String'[unresolved] :>> 'KerML::Root::Membership::memberShortName'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'ownedMemberName' : 'String'[unresolved] :>> 'KerML::Root::Membership::memberName'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived composite 'ownedMemberElement' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Relationship::ownedRelatedElement'[feature_def] :>> 'KerML::Root::Membership::memberElement'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def abstract 'Relationship' :> 'KerML::Root::Element'[metaclass_def]
          (feature_def 'isImplied' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def ordered 'target' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Relationship::relatedElement'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def ordered 'source' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Relationship::relatedElement'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def 'owningRelatedElement' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Relationship::relatedElement'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def composite ordered 'ownedRelatedElement' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Relationship::relatedElement'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'relatedElement' : 'KerML::Root::Element'[metaclass_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'TextualRepresentation' :> 'KerML::Root::AnnotatingElement'[metaclass_def]
          (feature_def 'language' : 'String'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'body' : 'String'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived 'representedElement' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Element::owner'[feature_def] :>> 'KerML::Root::AnnotatingElement::annotatedElement'[feature_def]
            (multiplicity_range [1..1])))
        (datatype_def 'VisibilityKind'
          (feature_def 'private' : 'KerML::Root::VisibilityKind'[datatype_def]
            (multiplicity_range [1]))
          (feature_def 'protected' : 'KerML::Root::VisibilityKind'[datatype_def]
            (multiplicity_range [1]))
          (feature_def 'public' : 'KerML::Root::VisibilityKind'[datatype_def]
            (multiplicity_range [1]))))
      (package 'Core'
        (namespace_import public -> 'KerML::Root'[package])
        (metaclass_def 'Classifier' :> 'KerML::Core::Type'[metaclass_def]
          (feature_def derived composite 'ownedSubclassification' : 'KerML::Core::Subclassification'[metaclass_def] :> 'KerML::Core::Type::ownedSpecialization'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'Conjugation' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'originalType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'conjugatedType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningType' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Core::Conjugation::conjugatedType'[feature_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'CrossSubsetting' :> 'KerML::Core::Subsetting'[metaclass_def]
          (feature_def 'crossedFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Subsetting::subsettedFeature'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'crossingFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Subsetting::owningFeature'[feature_def] :>> 'KerML::Core::Subsetting::subsettingFeature'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Differencing' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'differencingType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'typeDifferenced' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Disjoining' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'typeDisjoined' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'disjoiningType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningType' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :> 'KerML::Core::Disjoining::typeDisjoined'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'EndFeatureMembership' :> 'KerML::Core::FeatureMembership'[metaclass_def]
          (feature_def derived composite 'ownedMemberFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::FeatureMembership::ownedMemberFeature'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Feature' :> 'KerML::Core::Type'[metaclass_def]
          (feature_def 'isUnique' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isOrdered' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isComposite' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isEnd' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isDerived' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isPortion' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isVariable' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isConstant' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'direction' : 'KerML::Core::FeatureDirectionKind'[datatype_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'owningType' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Root::Element::owningNamespace'[feature_def] :> 'KerML::Core::Feature::featuringType'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'type' : 'KerML::Core::Type'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite 'ownedRedefinition' : 'KerML::Core::Redefinition'[metaclass_def] :> 'KerML::Core::Feature::ownedSubsetting'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite 'ownedSubsetting' : 'KerML::Core::Subsetting'[metaclass_def] :> 'KerML::Core::Type::ownedSpecialization'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'owningFeatureMembership' : 'KerML::Core::FeatureMembership'[metaclass_def] :> 'KerML::Root::Element::owningMembership'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'endOwningType' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Core::Feature::owningType'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived composite ordered 'ownedTyping' : 'KerML::Core::FeatureTyping'[metaclass_def] :> 'KerML::Core::Type::ownedSpecialization'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'featuringType' : 'KerML::Core::Type'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedTypeFeaturing' : 'KerML::Core::TypeFeaturing'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'chainingFeature' : 'KerML::Core::Feature'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite 'ownedFeatureInverting' : 'KerML::Core::FeatureInverting'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedFeatureChaining' : 'KerML::Core::FeatureChaining'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite 'ownedReferenceSubsetting' : 'KerML::Core::ReferenceSubsetting'[metaclass_def] :> 'KerML::Core::Feature::ownedSubsetting'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'featureTarget' : 'KerML::Core::Feature'[metaclass_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'crossFeature' : 'KerML::Core::Feature'[metaclass_def]
            (multiplicity_range [0..1]))
          (feature_def derived composite 'ownedCrossSubsetting' : 'KerML::Core::CrossSubsetting'[metaclass_def] :> 'KerML::Core::Feature::ownedSubsetting'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'FeatureChaining' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'chainingFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'featureChained' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1])))
        (datatype_def 'FeatureDirectionKind'
          (feature_def 'in' : 'KerML::Core::FeatureDirectionKind'[datatype_def]
            (multiplicity_range [1]))
          (feature_def 'inout' : 'KerML::Core::FeatureDirectionKind'[datatype_def]
            (multiplicity_range [1]))
          (feature_def 'out' : 'KerML::Core::FeatureDirectionKind'[datatype_def]
            (multiplicity_range [1])))
        (metaclass_def 'FeatureInverting' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'featureInverted' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'invertingFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::FeatureInverting::featureInverted'[feature_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'FeatureMembership' :> 'KerML::Root::OwningMembership'[metaclass_def]
          (feature_def derived 'owningType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Membership::membershipOwningNamespace'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived composite 'ownedMemberFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Root::OwningMembership::ownedMemberElement'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'FeatureTyping' :> 'KerML::Core::Specialization'[metaclass_def]
          (feature_def 'typedFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Specialization::specific'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'type' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Core::Specialization::general'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::FeatureTyping::typedFeature'[feature_def] :>> 'KerML::Core::Specialization::owningType'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'Intersecting' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'intersectingType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'typeIntersected' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Multiplicity' :> 'KerML::Core::Feature'[metaclass_def])
        (metaclass_def 'Redefinition' :> 'KerML::Core::Subsetting'[metaclass_def]
          (feature_def 'redefiningFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Subsetting::subsettingFeature'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'redefinedFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Subsetting::subsettedFeature'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'ReferenceSubsetting' :> 'KerML::Core::Subsetting'[metaclass_def]
          (feature_def 'referencedFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Subsetting::subsettedFeature'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'referencingFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Subsetting::owningFeature'[feature_def] :>> 'KerML::Core::Subsetting::subsettingFeature'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Specialization' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'general' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'specific' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningType' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :> 'KerML::Core::Specialization::specific'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'Subclassification' :> 'KerML::Core::Specialization'[metaclass_def]
          (feature_def 'superclassifier' : 'KerML::Core::Classifier'[metaclass_def] :>> 'KerML::Core::Specialization::general'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'subclassifier' : 'KerML::Core::Classifier'[metaclass_def] :>> 'KerML::Core::Specialization::specific'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningClassifier' : 'KerML::Core::Classifier'[metaclass_def] :>> 'KerML::Core::Specialization::owningType'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'Subsetting' :> 'KerML::Core::Specialization'[metaclass_def]
          (feature_def 'subsettedFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Specialization::general'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'subsettingFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Specialization::specific'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Subsetting::subsettingFeature'[feature_def] :>> 'KerML::Core::Specialization::owningType'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'Type' :> 'KerML::Root::Namespace'[metaclass_def]
          (feature_def 'isAbstract' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isSufficient' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived 'isConjugated' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived composite ordered 'ownedSpecialization' : 'KerML::Core::Specialization'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedFeatureMembership' : 'KerML::Core::FeatureMembership'[metaclass_def] :> 'KerML::Root::Namespace::ownedMembership'[feature_def] :> 'KerML::Core::Type::featureMembership'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'feature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Root::Namespace::member'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'ownedFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Root::Namespace::ownedMember'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'input' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Type::directedFeature'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'output' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Type::directedFeature'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'inheritedMembership' : 'KerML::Root::Membership'[metaclass_def] :> 'KerML::Root::Namespace::membership'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'endFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Type::feature'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'ownedEndFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Type::endFeature'[feature_def] :> 'KerML::Core::Type::ownedFeature'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite 'ownedConjugator' : 'KerML::Core::Conjugation'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'inheritedFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Type::feature'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'multiplicity' : 'KerML::Core::Multiplicity'[metaclass_def] :> 'KerML::Root::Namespace::ownedMember'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'unioningType' : 'KerML::Core::Type'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedIntersecting' : 'KerML::Core::Intersecting'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'intersectingType' : 'KerML::Core::Type'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedUnioning' : 'KerML::Core::Unioning'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite 'ownedDisjoining' : 'KerML::Core::Disjoining'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'featureMembership' : 'KerML::Core::FeatureMembership'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'differencingType' : 'KerML::Core::Type'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived composite ordered 'ownedDifferencing' : 'KerML::Core::Differencing'[metaclass_def] :> 'KerML::Root::Element::ownedRelationship'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'directedFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Type::feature'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'TypeFeaturing' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'featureOfType' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def 'featuringType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'owningFeatureOfType' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :> 'KerML::Core::TypeFeaturing::featureOfType'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'Unioning' :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def 'unioningType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'typeUnioned' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Root::Relationship::owningRelatedElement'[feature_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [1..1]))))
      (package 'Kernel'
        (namespace_import public -> 'KerML::Core'[package])
        (metaclass_def 'Association' :> 'KerML::Core::Classifier'[metaclass_def] :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def derived ordered 'relatedType' : 'KerML::Core::Type'[metaclass_def] :>> 'KerML::Root::Relationship::relatedElement'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'sourceType' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Kernel::Association::relatedType'[feature_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'targetType' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Kernel::Association::relatedType'[feature_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'associationEnd' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Type::endFeature'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'AssociationStructure' :> 'KerML::Kernel::Association'[metaclass_def] :> 'KerML::Kernel::Structure'[metaclass_def])
        (metaclass_def 'Behavior' :> 'KerML::Kernel::Class'[metaclass_def]
          (feature_def derived 'step' : 'KerML::Kernel::Step'[metaclass_def] :> 'KerML::Core::Type::feature'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'parameter' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Type::directedFeature'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'BindingConnector' :> 'KerML::Kernel::Connector'[metaclass_def])
        (metaclass_def 'BooleanExpression' :> 'KerML::Kernel::Expression'[metaclass_def]
          (feature_def derived 'predicate' : 'KerML::Kernel::Predicate'[metaclass_def] :>> 'KerML::Kernel::Expression::function'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'Class' :> 'KerML::Core::Classifier'[metaclass_def])
        (metaclass_def 'CollectExpression' :> 'KerML::Kernel::OperatorExpression'[metaclass_def]
          (feature_def 'operator' : 'String'[unresolved] :>> 'KerML::Kernel::OperatorExpression::operator'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Connector' :> 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Root::Relationship'[metaclass_def]
          (feature_def derived ordered 'relatedFeature' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Root::Relationship::relatedElement'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'association' : 'KerML::Kernel::Association'[metaclass_def] :>> 'KerML::Core::Feature::type'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'connectorEnd' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Type::endFeature'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'sourceFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Kernel::Connector::relatedFeature'[feature_def] :>> 'KerML::Root::Relationship::source'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'targetFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Kernel::Connector::relatedFeature'[feature_def] :>> 'KerML::Root::Relationship::target'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'defaultFeaturingType' : 'KerML::Core::Type'[metaclass_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'ConstructorExpression' :> 'KerML::Kernel::InstantiationExpression'[metaclass_def])
        (metaclass_def 'DataType' :> 'KerML::Core::Classifier'[metaclass_def])
        (metaclass_def 'ElementFilterMembership' :> 'KerML::Root::OwningMembership'[metaclass_def]
          (feature_def derived composite 'condition' : 'KerML::Kernel::Expression'[metaclass_def] :>> 'KerML::Root::OwningMembership::ownedMemberElement'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Expression' :> 'KerML::Kernel::Step'[metaclass_def]
          (feature_def derived 'isModelLevelEvaluable' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived 'function' : 'KerML::Kernel::Function'[metaclass_def] :>> 'KerML::Kernel::Step::behavior'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'result' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Type::output'[feature_def] :> 'KerML::Kernel::Step::parameter'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'FeatureChainExpression' :> 'KerML::Kernel::OperatorExpression'[metaclass_def]
          (feature_def 'operator' : 'String'[unresolved] :>> 'KerML::Kernel::OperatorExpression::operator'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived 'targetFeature' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Root::Namespace::member'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'FeatureReferenceExpression' :> 'KerML::Kernel::Expression'[metaclass_def]
          (feature_def derived 'referent' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Root::Namespace::member'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'FeatureValue' :> 'KerML::Root::OwningMembership'[metaclass_def]
          (feature_def 'isInitial' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def 'isDefault' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived 'featureWithValue' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Root::Membership::membershipOwningNamespace'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived composite 'value' : 'KerML::Kernel::Expression'[metaclass_def] :>> 'KerML::Root::OwningMembership::ownedMemberElement'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Flow' :> 'KerML::Kernel::Connector'[metaclass_def] :> 'KerML::Kernel::Step'[metaclass_def]
          (feature_def derived ordered 'payloadType' : 'KerML::Core::Classifier'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'targetInputFeature' : 'KerML::Core::Feature'[metaclass_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'sourceOutputFeature' : 'KerML::Core::Feature'[metaclass_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'flowEnd' : 'KerML::Kernel::FlowEnd'[metaclass_def] :> 'KerML::Kernel::Connector::connectorEnd'[feature_def]
            (multiplicity_range [0..2]))
          (feature_def derived 'payloadFeature' : 'KerML::Kernel::PayloadFeature'[metaclass_def] :> 'KerML::Core::Type::ownedFeature'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived ordered 'interaction' : 'KerML::Kernel::Interaction'[metaclass_def] :>> 'KerML::Kernel::Connector::association'[feature_def] :>> 'KerML::Kernel::Step::behavior'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'FlowEnd' :> 'KerML::Core::Feature'[metaclass_def])
        (metaclass_def 'Function' :> 'KerML::Kernel::Behavior'[metaclass_def]
          (feature_def derived 'isModelLevelEvaluable' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (feature_def derived 'expression' : 'KerML::Kernel::Expression'[metaclass_def] :> 'KerML::Kernel::Behavior::step'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'result' : 'KerML::Core::Feature'[metaclass_def] :> 'KerML::Core::Type::output'[feature_def] :> 'KerML::Kernel::Behavior::parameter'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'IndexExpression' :> 'KerML::Kernel::OperatorExpression'[metaclass_def]
          (feature_def 'operator' : 'String'[unresolved] :>> 'KerML::Kernel::OperatorExpression::operator'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def abstract 'InstantiationExpression' :> 'KerML::Kernel::Expression'[metaclass_def]
          (feature_def derived ordered 'argument' : 'KerML::Kernel::Expression'[metaclass_def]
            (multiplicity_range [0..*]))
          (feature_def derived 'instantiatedType' : 'KerML::Core::Type'[metaclass_def] :> 'KerML::Root::Namespace::member'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Interaction' :> 'KerML::Kernel::Association'[metaclass_def] :> 'KerML::Kernel::Behavior'[metaclass_def])
        (metaclass_def 'Invariant' :> 'KerML::Kernel::BooleanExpression'[metaclass_def]
          (feature_def 'isNegated' : 'Boolean'[unresolved]
            (multiplicity_range [1..1])))
        (metaclass_def 'InvocationExpression' :> 'KerML::Kernel::InstantiationExpression'[metaclass_def])
        (metaclass_def 'LibraryPackage' :> 'KerML::Kernel::Package'[metaclass_def]
          (feature_def 'isStandard' : 'Boolean'[unresolved]
            (multiplicity_range [1..1])))
        (metaclass_def 'LiteralBoolean' :> 'KerML::Kernel::LiteralExpression'[metaclass_def]
          (feature_def 'value' : 'Boolean'[unresolved]
            (multiplicity_range [1..1])))
        (metaclass_def 'LiteralExpression' :> 'KerML::Kernel::Expression'[metaclass_def])
        (metaclass_def 'LiteralInfinity' :> 'KerML::Kernel::LiteralExpression'[metaclass_def])
        (metaclass_def 'LiteralInteger' :> 'KerML::Kernel::LiteralExpression'[metaclass_def]
          (feature_def 'value' : 'Integer'[unresolved]
            (multiplicity_range [1..1])))
        (metaclass_def 'LiteralRational' :> 'KerML::Kernel::LiteralExpression'[metaclass_def]
          (feature_def 'value' : 'Rational'[unresolved]
            (multiplicity_range [1..1])))
        (metaclass_def 'LiteralString' :> 'KerML::Kernel::LiteralExpression'[metaclass_def]
          (feature_def 'value' : 'String'[unresolved]
            (multiplicity_range [1..1])))
        (metaclass_def 'Metaclass' :> 'KerML::Kernel::Structure'[metaclass_def])
        (metaclass_def 'MetadataAccessExpression' :> 'KerML::Kernel::Expression'[metaclass_def]
          (feature_def derived 'referencedElement' : 'KerML::Root::Element'[metaclass_def] :> 'KerML::Root::Namespace::member'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'MetadataFeature' :> 'KerML::Root::AnnotatingElement'[metaclass_def] :> 'KerML::Core::Feature'[metaclass_def]
          (feature_def derived 'metaclass' : 'KerML::Kernel::Metaclass'[metaclass_def] :> 'KerML::Core::Feature::type'[feature_def]
            (multiplicity_range [0..1])))
        (metaclass_def 'MultiplicityRange' :> 'KerML::Core::Multiplicity'[metaclass_def]
          (feature_def derived 'lowerBound' : 'KerML::Kernel::Expression'[metaclass_def] :> 'KerML::Kernel::MultiplicityRange::bound'[feature_def]
            (multiplicity_range [0..1]))
          (feature_def derived 'upperBound' : 'KerML::Kernel::Expression'[metaclass_def] :> 'KerML::Kernel::MultiplicityRange::bound'[feature_def]
            (multiplicity_range [1..1]))
          (feature_def derived ordered 'bound' : 'KerML::Kernel::Expression'[metaclass_def] :> 'KerML::Root::Namespace::ownedMember'[feature_def]
            (multiplicity_range [1..2])))
        (metaclass_def 'NullExpression' :> 'KerML::Kernel::Expression'[metaclass_def])
        (metaclass_def 'OperatorExpression' :> 'KerML::Kernel::InvocationExpression'[metaclass_def]
          (feature_def 'operator' : 'String'[unresolved]
            (multiplicity_range [1..1])))
        (metaclass_def 'Package' :> 'KerML::Root::Namespace'[metaclass_def]
          (feature_def derived ordered 'filterCondition' : 'KerML::Kernel::Expression'[metaclass_def] :> 'KerML::Root::Namespace::ownedMember'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'ParameterMembership' :> 'KerML::Core::FeatureMembership'[metaclass_def]
          (feature_def derived composite 'ownedMemberParameter' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::FeatureMembership::ownedMemberFeature'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'PayloadFeature' :> 'KerML::Core::Feature'[metaclass_def])
        (metaclass_def 'Predicate' :> 'KerML::Kernel::Function'[metaclass_def])
        (metaclass_def 'ResultExpressionMembership' :> 'KerML::Core::FeatureMembership'[metaclass_def]
          (feature_def derived composite 'ownedResultExpression' : 'KerML::Kernel::Expression'[metaclass_def] :>> 'KerML::Core::FeatureMembership::ownedMemberFeature'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'ReturnParameterMembership' :> 'KerML::Kernel::ParameterMembership'[metaclass_def])
        (metaclass_def 'SelectExpression' :> 'KerML::Kernel::OperatorExpression'[metaclass_def]
          (feature_def 'operator' : 'String'[unresolved] :>> 'KerML::Kernel::OperatorExpression::operator'[feature_def]
            (multiplicity_range [1..1])))
        (metaclass_def 'Step' :> 'KerML::Core::Feature'[metaclass_def]
          (feature_def derived ordered 'behavior' : 'KerML::Kernel::Behavior'[metaclass_def] :> 'KerML::Core::Feature::type'[feature_def]
            (multiplicity_range [0..*]))
          (feature_def derived ordered 'parameter' : 'KerML::Core::Feature'[metaclass_def] :>> 'KerML::Core::Type::directedFeature'[feature_def]
            (multiplicity_range [0..*])))
        (metaclass_def 'Structure' :> 'KerML::Kernel::Class'[metaclass_def])
        (metaclass_def 'Succession' :> 'KerML::Kernel::Connector'[metaclass_def])
        (metaclass_def 'SuccessionFlow' :> 'KerML::Kernel::Succession'[metaclass_def] :> 'KerML::Kernel::Flow'[metaclass_def])))))
~~~
