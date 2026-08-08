// Exercises the breadth of what the generator API can report: metaclasses, relationship
// kinds, multiplicities, documentation, flags and typing. Kept small enough that the golden
// transcript stays reviewable.
package Coverage {
    doc /* Package-level documentation. */

    attribute def Mass;
    attribute def Speed;

    part def Component {
        doc /* A base component. */
        attribute mass : Mass;
    }

    abstract part def Powered :> Component {
        attribute power;
    }

    part def Engine :> Powered {
        attribute mass : Mass;
        attribute speed : Speed[0..*] ordered;
    }

    part def Vehicle :> Component {
        part engine : Engine[1];
        ref part spare : Engine[0..1];
    }

    part vehicle : Vehicle;
}
