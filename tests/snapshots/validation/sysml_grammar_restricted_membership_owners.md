# META
~~~ini
description=Executable evidence that SysML grammar confines control nodes and subject, actor, stakeholder, objective and state-subaction memberships to their permitted owners: each spelling below places one in a forbidden owner and is rejected by the parser, so no such element reaches semantics
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=malformed
rule_family=validate
expectation=diagnostics
coverage_role=secondary
rule_id=sysml-2.0:8.3.17.6:validateControlNodeOwningType
rule_id=sysml-2.0:8.3.21.11:validateSubjectMembershipOwningType
rule_id=sysml-2.0:8.3.21.2:validateActorMembershipOwningType
rule_id=sysml-2.0:8.3.21.12:validateStakeholderMembershipOwningType
rule_id=sysml-2.0:8.3.22.4:validateObjectiveMembershipOwningType
rule_id=sysml-2.0:8.3.18.4:validateStateSubactionMembershipOwningType
type=file
~~~
# SOURCE
## control_node.sysml
~~~sysml
// ControlNode appears only through ActionNode in ActionBodyItem, whose owners are all action
// definitions or usages.
package P {
    part def Holder {
        fork f;
    }
}
~~~
## subject.sysml
~~~sysml
// SubjectMember appears only in RequirementBodyItem and CaseBodyItem.
package P {
    part def C;
    part def Holder {
        subject s : C;
    }
}
~~~
## actor.sysml
~~~sysml
// ActorMember appears only in RequirementBodyItem and CaseBodyItem.
package P {
    part def C;
    part def Holder {
        actor a : C;
    }
}
~~~
## stakeholder.sysml
~~~sysml
// StakeholderMember appears only in RequirementBodyItem.
package P {
    part def C;
    part def Holder {
        stakeholder s : C;
    }
}
~~~
## objective.sysml
~~~sysml
// ObjectiveMember appears only in CaseBodyItem, not in RequirementBodyItem.
package P {
    requirement def Holder {
        objective o;
    }
}
~~~
## state_subaction.sysml
~~~sysml
// EntryActionMember, DoActionMember and ExitActionMember appear only in StateBodyItem.
package P {
    part def Holder {
        entry action started;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/actor.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
  (document "memory://snapshot/control_node.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
  (document "memory://snapshot/objective.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 8) (end 4 4))
      )
    )
  )
  (document "memory://snapshot/stakeholder.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
  (document "memory://snapshot/state_subaction.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 8) (end 4 4))
      )
    )
  )
  (document "memory://snapshot/subject.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/actor.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
  (document "memory://snapshot/control_node.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
  (document "memory://snapshot/objective.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 8) (end 4 4))
      )
    )
  )
  (document "memory://snapshot/stakeholder.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
  (document "memory://snapshot/state_subaction.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 8) (end 4 4))
      )
    )
  )
  (document "memory://snapshot/subject.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:c1cc6c2f94c14179fbda8ad43363970943c6dd5cbe41d4bf5636682a19fa131c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/actor.sysml") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/actor.sysml") (qualified-name "P::C"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/actor.sysml") (qualified-name "P::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node.sysml") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node.sysml") (qualified-name "P::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/objective.sysml") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/objective.sysml") (qualified-name "P::Holder"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/stakeholder.sysml") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/stakeholder.sysml") (qualified-name "P::C"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/stakeholder.sysml") (qualified-name "P::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_subaction.sysml") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_subaction.sysml") (qualified-name "P::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/subject.sysml") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/subject.sysml") (qualified-name "P::C"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/subject.sysml") (qualified-name "P::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
