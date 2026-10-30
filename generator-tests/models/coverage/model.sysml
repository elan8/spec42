// Exercises the breadth of what the generator API can report: metaclasses, relationship
// kinds, multiplicities, documentation, flags and typing. Kept small enough that the golden
// transcript stays reviewable.
package Coverage {
    doc /* Package-level documentation. */

    attribute booleanLiteral = true;
    attribute booleanText = "true";
    attribute integerLiteral = 1;
    attribute integerText = "1";
    attribute realLiteral = 1.0;

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
