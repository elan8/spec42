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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ker_ml.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 131 16) (end 131 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 297 16) (end 297 20))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d8fa15e3ed0a81b77992a7a038f2c5106b08b5bf07dc3d05cdd95ede54e8bf6e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "KerML"))) (kind "package") (name "KerML") (declared-name "KerML"))
    (element (id (node (document "d0") (qualified-name "KerML::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "KerML"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "KerML::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "KerML"))) (authored (membership (kind Import) (visibility "public") (import (reference "Kernel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "KerML::Core"))) (kind "package") (name "Core") (declared-name "Core") (parent (node (document "d0") (qualified-name "KerML"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "KerML::Core"))) (authored (membership (kind Import) (visibility "public") (import (reference "Root::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Classifier"))) (kind "kermlDecl") (name "Classifier") (declared-name "Classifier") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Conjugation"))) (kind "kermlDecl") (name "Conjugation") (declared-name "Conjugation") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::CrossSubsetting"))) (kind "kermlDecl") (name "CrossSubsetting") (declared-name "CrossSubsetting") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Differencing"))) (kind "kermlDecl") (name "Differencing") (declared-name "Differencing") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Disjoining"))) (kind "kermlDecl") (name "Disjoining") (declared-name "Disjoining") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::EndFeatureMembership"))) (kind "kermlDecl") (name "EndFeatureMembership") (declared-name "EndFeatureMembership") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Feature"))) (kind "kermlDecl") (name "Feature") (declared-name "Feature") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::FeatureChaining"))) (kind "kermlDecl") (name "FeatureChaining") (declared-name "FeatureChaining") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::FeatureDirectionKind"))) (kind "kermlDecl") (name "FeatureDirectionKind") (declared-name "FeatureDirectionKind") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::FeatureInverting"))) (kind "kermlDecl") (name "FeatureInverting") (declared-name "FeatureInverting") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::FeatureMembership"))) (kind "kermlDecl") (name "FeatureMembership") (declared-name "FeatureMembership") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::FeatureTyping"))) (kind "kermlDecl") (name "FeatureTyping") (declared-name "FeatureTyping") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Intersecting"))) (kind "kermlDecl") (name "Intersecting") (declared-name "Intersecting") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Multiplicity"))) (kind "kermlDecl") (name "Multiplicity") (declared-name "Multiplicity") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Redefinition"))) (kind "kermlDecl") (name "Redefinition") (declared-name "Redefinition") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::ReferenceSubsetting"))) (kind "kermlDecl") (name "ReferenceSubsetting") (declared-name "ReferenceSubsetting") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Specialization"))) (kind "kermlDecl") (name "Specialization") (declared-name "Specialization") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Subclassification"))) (kind "kermlDecl") (name "Subclassification") (declared-name "Subclassification") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Subsetting"))) (kind "kermlDecl") (name "Subsetting") (declared-name "Subsetting") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Type"))) (kind "kermlDecl") (name "Type") (declared-name "Type") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::TypeFeaturing"))) (kind "kermlDecl") (name "TypeFeaturing") (declared-name "TypeFeaturing") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Core::Unioning"))) (kind "kermlDecl") (name "Unioning") (declared-name "Unioning") (parent (node (document "d0") (qualified-name "KerML::Core"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel"))) (kind "package") (name "Kernel") (declared-name "Kernel") (parent (node (document "d0") (qualified-name "KerML"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "KerML::Kernel"))) (authored (membership (kind Import) (visibility "public") (import (reference "Core::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Association"))) (kind "kermlDecl") (name "Association") (declared-name "Association") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::AssociationStructure"))) (kind "kermlDecl") (name "AssociationStructure") (declared-name "AssociationStructure") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Behavior"))) (kind "kermlDecl") (name "Behavior") (declared-name "Behavior") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::BindingConnector"))) (kind "kermlDecl") (name "BindingConnector") (declared-name "BindingConnector") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::BooleanExpression"))) (kind "kermlDecl") (name "BooleanExpression") (declared-name "BooleanExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Class"))) (kind "kermlDecl") (name "Class") (declared-name "Class") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::CollectExpression"))) (kind "kermlDecl") (name "CollectExpression") (declared-name "CollectExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Connector"))) (kind "kermlDecl") (name "Connector") (declared-name "Connector") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::ConstructorExpression"))) (kind "kermlDecl") (name "ConstructorExpression") (declared-name "ConstructorExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::DataType"))) (kind "kermlDecl") (name "DataType") (declared-name "DataType") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::ElementFilterMembership"))) (kind "kermlDecl") (name "ElementFilterMembership") (declared-name "ElementFilterMembership") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Expression"))) (kind "kermlDecl") (name "Expression") (declared-name "Expression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::FeatureChainExpression"))) (kind "kermlDecl") (name "FeatureChainExpression") (declared-name "FeatureChainExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::FeatureReferenceExpression"))) (kind "kermlDecl") (name "FeatureReferenceExpression") (declared-name "FeatureReferenceExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::FeatureValue"))) (kind "kermlDecl") (name "FeatureValue") (declared-name "FeatureValue") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Flow"))) (kind "kermlDecl") (name "Flow") (declared-name "Flow") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::FlowEnd"))) (kind "kermlDecl") (name "FlowEnd") (declared-name "FlowEnd") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Function"))) (kind "kermlDecl") (name "Function") (declared-name "Function") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::IndexExpression"))) (kind "kermlDecl") (name "IndexExpression") (declared-name "IndexExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::InstantiationExpression"))) (kind "kermlDecl") (name "InstantiationExpression") (declared-name "InstantiationExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Interaction"))) (kind "kermlDecl") (name "Interaction") (declared-name "Interaction") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Invariant"))) (kind "kermlDecl") (name "Invariant") (declared-name "Invariant") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::InvocationExpression"))) (kind "kermlDecl") (name "InvocationExpression") (declared-name "InvocationExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::LibraryPackage"))) (kind "kermlDecl") (name "LibraryPackage") (declared-name "LibraryPackage") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralBoolean"))) (kind "kermlDecl") (name "LiteralBoolean") (declared-name "LiteralBoolean") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralExpression"))) (kind "kermlDecl") (name "LiteralExpression") (declared-name "LiteralExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralInfinity"))) (kind "kermlDecl") (name "LiteralInfinity") (declared-name "LiteralInfinity") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralInteger"))) (kind "kermlDecl") (name "LiteralInteger") (declared-name "LiteralInteger") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralRational"))) (kind "kermlDecl") (name "LiteralRational") (declared-name "LiteralRational") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::LiteralString"))) (kind "kermlDecl") (name "LiteralString") (declared-name "LiteralString") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Metaclass"))) (kind "kermlDecl") (name "Metaclass") (declared-name "Metaclass") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::MetadataAccessExpression"))) (kind "kermlDecl") (name "MetadataAccessExpression") (declared-name "MetadataAccessExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::MetadataFeature"))) (kind "kermlDecl") (name "MetadataFeature") (declared-name "MetadataFeature") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::MultiplicityRange"))) (kind "kermlDecl") (name "MultiplicityRange") (declared-name "MultiplicityRange") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::NullExpression"))) (kind "kermlDecl") (name "NullExpression") (declared-name "NullExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::OperatorExpression"))) (kind "kermlDecl") (name "OperatorExpression") (declared-name "OperatorExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Package"))) (kind "kermlDecl") (name "Package") (declared-name "Package") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::ParameterMembership"))) (kind "kermlDecl") (name "ParameterMembership") (declared-name "ParameterMembership") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::PayloadFeature"))) (kind "kermlDecl") (name "PayloadFeature") (declared-name "PayloadFeature") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Predicate"))) (kind "kermlDecl") (name "Predicate") (declared-name "Predicate") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::ResultExpressionMembership"))) (kind "kermlDecl") (name "ResultExpressionMembership") (declared-name "ResultExpressionMembership") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::ReturnParameterMembership"))) (kind "kermlDecl") (name "ReturnParameterMembership") (declared-name "ReturnParameterMembership") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::SelectExpression"))) (kind "kermlDecl") (name "SelectExpression") (declared-name "SelectExpression") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Step"))) (kind "kermlDecl") (name "Step") (declared-name "Step") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Structure"))) (kind "kermlDecl") (name "Structure") (declared-name "Structure") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::Succession"))) (kind "kermlDecl") (name "Succession") (declared-name "Succession") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Kernel::SuccessionFlow"))) (kind "kermlDecl") (name "SuccessionFlow") (declared-name "SuccessionFlow") (parent (node (document "d0") (qualified-name "KerML::Kernel"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root"))) (kind "package") (name "Root") (declared-name "Root") (parent (node (document "d0") (qualified-name "KerML"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::AnnotatingElement"))) (kind "kermlDecl") (name "AnnotatingElement") (declared-name "AnnotatingElement") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Annotation"))) (kind "kermlDecl") (name "Annotation") (declared-name "Annotation") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Comment"))) (kind "kermlDecl") (name "Comment") (declared-name "Comment") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Dependency"))) (kind "kermlDecl") (name "Dependency") (declared-name "Dependency") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Documentation"))) (kind "kermlDecl") (name "Documentation") (declared-name "Documentation") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Element"))) (kind "kermlDecl") (name "Element") (declared-name "Element") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Import"))) (kind "kermlDecl") (name "Import") (declared-name "Import") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Membership"))) (kind "kermlDecl") (name "Membership") (declared-name "Membership") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::MembershipImport"))) (kind "kermlDecl") (name "MembershipImport") (declared-name "MembershipImport") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Namespace"))) (kind "kermlDecl") (name "Namespace") (declared-name "Namespace") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::NamespaceImport"))) (kind "kermlDecl") (name "NamespaceImport") (declared-name "NamespaceImport") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::OwningMembership"))) (kind "kermlDecl") (name "OwningMembership") (declared-name "OwningMembership") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::Relationship"))) (kind "kermlDecl") (name "Relationship") (declared-name "Relationship") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::TextualRepresentation"))) (kind "kermlDecl") (name "TextualRepresentation") (declared-name "TextualRepresentation") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::Root::VisibilityKind"))) (kind "kermlDecl") (name "VisibilityKind") (declared-name "VisibilityKind") (parent (node (document "d0") (qualified-name "KerML::Root"))))
    (element (id (node (document "d0") (qualified-name "KerML::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "KerML"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "KerML::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "KerML::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Kernel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "KerML::Kernel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "KerML::Core::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Root::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "KerML::Kernel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Core::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 131 16) (end 131 20)) (probe (position 131 16))
      (reference
        (source (document "d0") (qualified-name "KerML::Core::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Root::*")
        (range (start 131 16) (end 131 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 297 16) (end 297 20)) (probe (position 297 16))
      (reference
        (source (document "d0") (qualified-name "KerML::Kernel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Core::*")
        (range (start 297 16) (end 297 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 15) (end 7 21)) (probe (position 7 15))
      (reference
        (source (document "d0") (qualified-name "KerML::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Kernel::*")
        (range (start 7 15) (end 7 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "KerML::Kernel") (range (start 296 1) (end 296 6977)))
        )
      )
    )
    (query (range (start 6 16) (end 6 28)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "KerML::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 6 16) (end 6 28))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
