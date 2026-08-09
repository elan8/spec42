# META
~~~ini
description=Standard Library: Domain Libraries/Geometry/ShapeItems
type=file
~~~
# SOURCE
~~~sysml
standard library package ShapeItems {
	doc
	/*
	 * This package provides a model of items that represent basic geometric shapes. 
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::Positive;
	private import ISQSpaceTime::*;
	private import ISQBase::*;
	private import SI::m;
	private import Occurrences::MatesWith;
	private import Objects::*;
	private import Items::Item;
	private import SequenceFunctions::equals;
	private import SequenceFunctions::isEmpty;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::size;
	private import SequenceFunctions::includes;
	private import ControlFunctions::'if';
	private import ControlFunctions::forAll;
	private import ControlFunctions::exists;
	private import Quantities::scalarQuantities;

	item def PlanarCurve :> Curve {
		doc
		/*
		 * A PlanarCurve is a Curve with a given length embeddable in a plane.
		 */
	
		attribute :>> length [1];

		attribute :>> outerSpaceDimension;
		assert constraint { notEmpty(outerSpaceDimension) &  outerSpaceDimension <= 2 }
	}

	item def PlanarSurface :> Surface {
		doc
		/*
		 * A PlanarSurface is a flat Surface with a given area.
		 */
	
		attribute :>> area [1];
		attribute :>> outerSpaceDimension = 2;

		item :>> shape : PlanarCurve;
	}

	item def Line :> PlanarCurve {
		doc
		/*
		 * A Line is a Curve that is a straight line of a given length.
		 */
	
		attribute :>> length [1];
		attribute :>> outerSpaceDimension = 1;
	}

	abstract item def Path :> StructuredSpaceObject::StructuredCurve {
		doc
		/*
		 * Path is the most general structured Curve.
		 */
        
		item :>> faces [0];
		item :>> edges [1..*] {
			item :>> vertices [0..2];
		}
		item :>> vertices [*] = edges.vertices;

		assert constraint { isClosed == vertices->forAll{in p1 : Point;
					vertices->exists{p2 : Point; p1 != p2 and
							 includes(p1.matingOccurrences, p2) } } }
	}

	attribute semiMajorAxis : LengthValue [0..*] :> scalarQuantities;
	attribute semiMinorAxis : LengthValue [0..*] :> scalarQuantities;
	attribute xoffset : LengthValue [0..*] :> scalarQuantities default 0 [m];
	attribute yoffset : LengthValue [0..*] :> scalarQuantities default 0 [m];
	attribute baseLength : LengthValue [0..*] :> scalarQuantities;
	attribute baseWidth : LengthValue [0..*] :> scalarQuantities;

	item def ConicSection :> Path, PlanarCurve {
		doc
		/*
		 * A ConicSection is a closed PlanarCurve, possibly disconnected, see Hyperbola.
		 */
	

		item :>> edges [1..2];

		item :>> vertices [0];
	}

	item def Ellipse :> ConicSection {
		doc
		/*
		 * An Ellipse is a ConicSection in the shape of an ellipse of a given semiaxes.
		 */
	
		attribute :>> semiMajorAxis [1];
		attribute :>> semiMinorAxis [1];

		item :>> edges [1];
	}

	item def Circle :> Ellipse {
		doc
		/*
		 * A Circle is an Ellipse with semiaxes equal to its radius.
		 */
	
		attribute :>> radius [1];
		attribute :>> semiMajorAxis [1] = radius;
		attribute :>> semiMinorAxis [1] = radius;

		item :>> edges {
			attribute length [1] = Circle::radius * TrigFunctions::pi * 2;
		}
	}

	item def Parabola :> ConicSection {
		doc
		/*
		 * A Parabola is a ConicSection in the shape of a parabola of a given focal length.
		 */
	
		attribute focalDistance : LengthValue [1] :> scalarQuantities;

		item :>> edges [1];
	}

	item def Hyperbola :> ConicSection {
		doc
		/*
		 * A Hyperbola is a ConicSection in the shape of a hyperbola with given axes.
		 */
	
		attribute tranverseAxis : LengthValue [1] :> scalarQuantities;
		attribute conjugateAxis : LengthValue [1] :> scalarQuantities;
	}

	item def Polygon :> Path, PlanarCurve {
		doc
		/*
		 * A Polygon is a closed planar Path with straight edges.
		 */
	
		item :>> edges : Line { item :>> vertices [2]; }

		attribute :>> isClosed = true;

		assert constraint { (1..size(edges))->forAll {in i;
					edges#(i).vertices->equals((vertices#((2*i)-1), vertices#(2*i))) and  
					includes((edges#(i).vertices#(2) as Item).matingOccurrences,
						 edges#(if i==size(edges) ? 1 else i+1).vertices#(1)) } }
	}

	item def Triangle :> Polygon {
		doc
		/*
		 * A Triangle is three-sided Polygon  with given length (base), width (perpendicular distance
		 * from base to apex), and offset of this perpendicular at the base from the center of the base.
		 */
	
		attribute :>> length [1];
		attribute :>> width [1];
		attribute :>> xoffset [1];

		item :>> edges [3] = (base, e2, e3);
		item base [1] { length = Triangle::length; }
		item e2 [1];
		item e3 [1];

		item :>> vertices [6];
		item v12  [2] ordered = (vertices#(2), vertices#(3));
		item apex [2] ordered = (vertices#(4), vertices#(5));
		item v31  [2] ordered = (vertices#(6), vertices#(1));
	}

	item def RightTriangle :> Triangle {
		doc
		/*
		 * A RightTriangle is a Triangle with sides opposite the hypotenuse at right angles.
		 */
	
		attribute :>> xoffset = length / 2;

		item :>> e2 { attribute :>> length = Triangle::width; }

		item hypotenuse :>> e3 {
			attribute :>> length = ( Triangle::length^2 + Triangle::width^2 );
		}
	}

	item def Quadrilateral :> Polygon {
		doc
		/*
		 * A Quadrilateral is a four-sided Polygon.
		 */
	
		item :>> edges [4] = (e1, e2, e3, e4);
		item e1 [1];
		item e2 [1];
		item e3 [1];
		item e4 [1];

		item :>> vertices [8];
		item v12 [2] ordered = (vertices#(2), vertices#(3));
		item v23 [2] ordered = (vertices#(4), vertices#(5));
		item v34 [2] ordered = (vertices#(6), vertices#(7));
		item v41 [2] ordered = (vertices#(6), vertices#(1));
	}

	item def Rectangle :> Quadrilateral {
		doc
		/*
		 * A Rectangle is a Quadrilateral four right angles and given length and width.
		 */
	
		attribute :>> length [1];
		attribute :>> width [1];

		item :>> e1 { attribute :>> length = Rectangle::length; }
		item :>> e2 { attribute :>> length = Rectangle::width; }
		item :>> e3 { attribute :>> length = e1.length; }
		item :>> e4 { attribute :>> length = e2.length; }
	}

	abstract item def Shell :> StructuredSpaceObject::StructuredSurface {
		doc
		/*
		 * Shell is the most general structured Surface.
		 */
	}

	item def Disc :> Shell, PlanarSurface {
		doc
		/*
		 * A Disc is a Shell bound by an Ellipse.
		 */
	
		attribute :>> semiMajorAxis [1];
		attribute :>> semiMinorAxis [1];

		item :>> shape : Ellipse [1] {
			attribute :>> semiMajorAxis = Disc::semiMajorAxis;
			attribute :>> semiMinorAxis = Disc::semiMinorAxis;
		}

		item :>> faces : PlanarSurface [1] {
			item :>> edges [1];
		}
		item :>> edges : Ellipse [1] = shape {
            attribute :>> Shell::edges::innerSpaceDimension, Ellipse::innerSpaceDimension;
            ref item :>> Shell::edges::vertices, Ellipse::vertices;
		}
		item :>> vertices [0];
	}

	item def CircularDisc :> Disc {
		doc
		/*
		 * A CircularDisc is a Disc bound by a Circle.
		 */
	
		attribute :>> radius [1];
		attribute :>> semiMajorAxis [1] = radius;
		attribute :>> semiMinorAxis [1] = radius;

		item :>> shape : Circle {
            attribute :>> Disc::shape::semiMajorAxis, Circle::semiMajorAxis;
            attribute :>> Disc::shape::semiMinorAxis, Circle::semiMinorAxis;
        }
		item :>> edges : Circle;
	}

	item def ConicSurface :> Shell {
		doc
		/*
		 * A ConicSurface is a Surface that has ConicSection cross-sections.
		 */
	
		item :>> faces [1..2];
		item :>> edges [0];
		item :>> vertices [0];

		attribute :>> genus = 0;
	}

	item def Ellipsoid :> ConicSurface {
		doc
		/*
		 * An Ellipsoid is a ConicSurface with only elliptical cross-sections.
		 */
	
		attribute semiAxis1 : LengthValue [1] :> scalarQuantities; 
		attribute semiAxis2 : LengthValue [1] :> scalarQuantities;
		attribute semiAxis3 : LengthValue [1] :> scalarQuantities;

		item :>> faces [1];
	}

	item def Sphere :> Ellipsoid {
		doc
		/*
		 * A Sphere is an Ellipsoid with all the same semiaxes.
		 */	

		attribute :>> radius [1];
		attribute :>> semiAxis1 [1] = radius;
		attribute :>> semiAxis2 [1] = radius;
		attribute :>> semiAxis3 [1] = radius;
	}

	item def Paraboloid :> ConicSurface {
		doc
		/*
		 * A Paraboloid is a ConicSurface with only parabolic cross-sections.
		 */
	
		attribute focalDistance : LengthValue [1] :> scalarQuantities;

		item :>> faces [1];
	}

	item def Hyperboloid :> ConicSurface {
		doc
		/*
		 * A Hyperboloid is a ConicSurface with only hyperbolic cross-sections.
		 */
	
		attribute transverseAxis : LengthValue [1] :> scalarQuantities;
		attribute conjugateAxis : LengthValue [1] :> scalarQuantities;
	}

	item def Toroid :> Shell {
		doc
		/*
		 * A Toroid is a surface generated from revolving a planar closed curve about an line coplanar
		 * with the curve. It is single sided with one hole.
		 */	

		attribute revolutionRadius : LengthValue [1] :> scalarQuantities;

		item revolvedCurve : PlanarCurve [1] { attribute :>> isClosed = true; }

		item :>> faces [1];
		item :>> edges [0];
		item :>> vertices [0];

		attribute :>> genus = 1;
	}

	item def Torus :> Toroid {
		doc
		/*
		 * A Torus is a revolution of a Circle.
		 */	

		attribute majorRadius :>> revolutionRadius;
		attribute minorRadius : LengthValue [1] :> scalarQuantities;

		item :>> revolvedCurve: Circle [1] { attribute :>> radius = minorRadius; }
	}


	item def RectangularToroid :> Toroid {
		doc
		/*
		 * A RectangularToroid is a revolution of a Rectangle.
		 */	

		attribute rectangleLength : LengthValue [1] :> scalarQuantities;
		attribute rectangleWidth  : LengthValue [1] :> scalarQuantities;

		item :>> revolvedCurve: Rectangle [1] {
			attribute :>> length = rectangleLength;
			attribute :>> width  = rectangleWidth;
			attribute :>> revolvedCurve::isClosed, Rectangle::isClosed;
		}
	}

	item def ConeOrCylinder :> Shell {
		doc
		/*
		 * A ConeOrCylinder is Shell that a Cone or a Cylinder with a given elliptical base,
		 * height, width (perpendicular distance from the base to the center of the top side or vertex),
		 * and offsets of this perpendicular at the base from the center of the base.
		 */
	
		attribute :>> semiMajorAxis [1];
		attribute :>> semiMinorAxis [1];
		attribute :>> height [1];

		attribute :>> xoffset [1];
		attribute :>> yoffset [1];

		item :>> faces [2..3];
		item base : Disc [1] :> faces {        
            attribute :>> Disc::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Disc::edges, ConeOrCylinder::faces::edges {
                attribute :>> Disc::edges::innerSpaceDimension, ConeOrCylinder::faces::edges::innerSpaceDimension;
            }
            ref :>> Disc::vertices, ConeOrCylinder::faces::vertices;		    
		}
		item af : Disc [0..1] :> faces {        
            attribute :>> Disc::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Disc::edges, ConeOrCylinder::faces::edges {
                attribute :>> Disc::edges::innerSpaceDimension, ConeOrCylinder::faces::edges::innerSpaceDimension;
            }
            ref :>> Disc::vertices, ConeOrCylinder::faces::vertices;            
        }
		item cf : Surface [1] :> faces;

		item :>> edges [2..4] = faces.edges;
		item be [2] :> edges { 
			attribute :>> semiMajorAxis = ConeOrCylinder::semiMajorAxis;
			attribute :>> semiMinorAxis = ConeOrCylinder::semiMinorAxis;
		}
		item ae [0..2] :> edges {
			attribute :>> semiMajorAxis = be.semiMajorAxis;
			attribute :>> semiMinorAxis = be.semiMinorAxis;
		}
		assert constraint { size(ae) == (if isEmpty(af) ? 0 else 2) and
				            size(edges) == (if isEmpty(af) ? 2 else 4)  }

		item :>> vertices [0..1] = faces.vertices;
		assert constraint { isEmpty(af) == notEmpty(vertices) }

		/* Bind face edges to specific edges */
		binding [1] bind [0..*] base.edges = [0..*] be;
		binding [1] bind [0..*] cf.edges = [0..*] be;

		/* Meeting edges */
		connection :MatesWith connect [1] be to [1] be;

		attribute :>> genus = 0;
	}

	item def Cone :> ConeOrCylinder {
		doc
		/*
		 * A Cone has one elliptical sides joined to a point by a curved side.
		 */	

		item :>> faces [2];

		item apex :>> vertices;

		/* Bind face vertices to specific vertices */
		binding [1] bind [0..*] cf.vertices = [0..*] apex;
	}

	item def EccentricCone :> Cone {
		doc
		/*
		 * An EccentricCone is a Cone with least one positive offset.
		 */
	
		assert constraint { xoffset > 0 or yoffset > 0 }
	}

	item def CircularCone :> Cone {
		doc
		/*
		 * A CircularCone is a Cone with a circular base.
		 */	

		attribute :>> radius [1];
		attribute :>> semiMajorAxis [1] = radius;
		attribute :>> semiMinorAxis [1] = radius;

		item :>> base : CircularDisc {
		    ref :>> base::edges, CircularDisc::edges;
		}
	}

	item def RightCircularCone :> CircularCone {
		doc
		/*
		 * A RightCircularCone is a CircularCone with zero offsets.
		 */
	
		attribute :>> xoffset { attribute :>> num = 0; }
		attribute :>> yoffset { attribute :>> num = 0; }
	}

	item def Cylinder :> ConeOrCylinder {
		doc
		/*
		 * A Cylinder has two elliptical sides joined by a curved side.
		 */
	
		item :>> af [1];

		binding [1] bind [0..*] cf.edges = [0..*] ae;

		connection :MatesWith connect [1] ae to [1] ae {
			doc /* Meeting edges */
		}
	}

	item def EccentricCylinder :> Cylinder {
	doc
	/*
	 * An EccentricCylinder is a Cylinder with least one positive offset.
	 */
	
		assert constraint { xoffset > 0 or yoffset > 0 }
	}

	item def CircularCylinder :> Cylinder {
		doc
		/*
		 * A CircularCylinder is a Cylinder with two circular sides.
		 */
	
		attribute :>> radius [1];
		attribute :>> semiMajorAxis [1] = radius;
		attribute :>> semiMinorAxis [1] = radius;

		item :>> base : CircularDisc {
            ref :>> base::edges, CircularDisc::edges;
        }
		item :>> af : CircularDisc {
            ref :>> af::edges, CircularDisc::edges;
        }
	}

	item def RightCircularCylinder :> CircularCylinder {
		doc
		/*
		 * A RightCircularCylinder is a CircularCylinder with zero offsets.
		 */
	
		attribute :>> xoffset { attribute :>> num = 0; }
		attribute :>> yoffset { attribute :>> num = 0; }
	}

	item def Polyhedron :> Shell {
		doc
		/*
		 * A Polyhedron is a closed Shell with polygonal sides.
		 */	

		attribute :>> isClosed = true;

		item :>> faces : Polygon [2..*] {        
            attribute :>> Polygon::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Polygon::edges, ConeOrCylinder::faces::edges;
            ref :>> Polygon::vertices, ConeOrCylinder::faces::vertices;            
        }
		
		item :>> edges = faces.edges;
		
		attribute :>> outerSpaceDimension = if size(faces) > 2 ? 3 else 2;

		attribute :>> genus = 0;
	}

	item def CuboidOrTriangularPrism :> Polyhedron {
		doc
		/*
		 * A CuboidOrTriangularPrism is a Polyhedron that is either a Cuboid or TriangularPrism.
		 */

		item :>> faces [5..6];
		item tf	 : Quadrilateral [1] :> faces {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }
		item bf	 : Quadrilateral [1] :> faces {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }
		item ff	 : Polygon [1] :> faces { item :>> Polygon::edges, faces::edges [3..4]; }
		item rf	 : Polygon [1] :> faces { item :>> Polygon::edges, faces::edges [3..4]; }
		item slf : Quadrilateral [1] :> faces {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }
		item srf : Quadrilateral [0..1] :> faces {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }

		item :>> edges;
		assert constraint { size(edges) == 18 or size(edges) == 24 }
		
		item tfe  [2]	 :> edges;
		item tre  [2]	 :> edges;
		item tsle [2]	 :> edges;
		item tsre [0..2] :> edges;
		item bfe  [2]	 :> edges;
		item bre  [2]	 :> edges;
		item bsle [2]	 :> edges;
		item bsre [2]	 :> edges;
		item ufle [2]	 :> edges;
		item ufre [0..2] :> edges;
		item urle [2]	 :> edges;
		item urre [0..2] :> edges;

		assert constraint { ( isEmpty(srf) implies isEmpty(tsre) ) and
				    ( isEmpty(tsre) == isEmpty(ufre) ) and
				    ( isEmpty(ufre) == isEmpty(urre) ) }

		item :>> vertices;
		assert constraint { size(vertices) == size(edges) }

		item tflv [3]	 :> vertices;
		item tfrv [0..3] :> vertices;
		item trlv [3]	 :> vertices;
		item trrv [0..3] :> vertices;
		item bflv [3]	 :> vertices;
		item bfrv [3]	 :> vertices;
		item brlv [3]	 :> vertices;
		item brrv [3]	 :> vertices;
		
		assert constraint { ( isEmpty(tfrv) == isEmpty(trrv) ) }

		/* Bind face edges to specific edges */
		binding [1] bind [0..1] tf.edges = [0..1] tfe;
		binding [1] bind [0..1] tf.edges = [0..1] tre;
		binding [1] bind [0..1] tf.edges = [0..1] tsle;
		binding [1] bind [0..1] bf.edges = [0..1] bfe;
		binding [1] bind [0..1] bf.edges = [0..1] bre;
		binding [1] bind [0..1] bf.edges = [0..1] bsle;
		binding [1] bind [0..1] bf.edges = [0..1] bsre;

		binding [1] bind [0..1] ff.edges = [0..1] tfe;
		binding [1] bind [0..1] ff.edges = [0..1] bfe;
		binding [1] bind [0..1] ff.edges = [0..1] ufle;

		binding [1] bind [0..1] rf.edges = [0..1] tre;
		binding [1] bind [0..1] rf.edges = [0..1] bre;
		binding [1] bind [0..1] rf.edges = [0..1] urle;

		/* Bind edge vertices to specific vertices */
		binding [1] bind [0..1] tfe.vertices = [0..1] tflv;
		binding [1] bind [0..1] tre.vertices = [0..1] trlv;
		binding [1] bind [0..1] tsle.vertices = [0..1] tflv;
		binding [1] bind [0..1] tsle.vertices = [0..1] trlv;

		binding [1] bind [0..1] bfe.vertices = [0..1] bflv;
		binding [1] bind [0..1] bfe.vertices = [0..1] bfrv;
		binding [1] bind [0..1] bre.vertices = [0..1] brlv;
		binding [1] bind [0..1] bre.vertices = [0..1] brrv;
		binding [1] bind [0..1] bsle.vertices = [0..1] bflv;
		binding [1] bind [0..1] bsle.vertices = [0..1] brlv;
		binding [1] bind [0..1] bsre.vertices = [0..1] bfrv;
		binding [1] bind [0..1] bsre.vertices = [0..1] brrv;

		binding [1] bind [0..1] ufle.vertices = [0..1] tflv;
		binding [1] bind [0..1] ufle.vertices = [0..1] bflv;
		binding [1] bind [0..1] urle.vertices = [0..1] trlv;
		binding [1] bind [0..1] urle.vertices = [0..1] brlv;

		/* Meeting edges */
		connection :MatesWith connect [1] tfe to [1] tfe;
		connection :MatesWith connect [1] tre to [1] tre;
		connection :MatesWith connect [1] tsle to [1] tsle;
		connection :MatesWith connect [1] bfe to [1] bfe;
		connection :MatesWith connect [1] bre to [1] bre;
		connection :MatesWith connect [1] bsle to [1] bsle;
		connection :MatesWith connect [1] bsre to [1] bsre;
		connection :MatesWith connect [1] ufle to [1] ufle;
		connection :MatesWith connect [1] urle to [1] urle;
		connection :MatesWith connect [1] bsre to [1] bsre;

		/* Meeting vertices  */
		connection :MatesWith connect [2] tflv to [2] tflv;
		connection :MatesWith connect [2] trlv to [2] trlv;
		connection :MatesWith connect [2] bflv to [2] bflv;
		connection :MatesWith connect [2] bfrv to [2] bfrv;
		connection :MatesWith connect [2] brlv to [2] brlv;
		connection :MatesWith connect [2] brrv to [2] brrv;
	}

	item def TriangularPrism :> CuboidOrTriangularPrism {
		doc
		/*
		 * A TriangularPrism is a Polyhedron with five sides, two triangular and
		 * the others quadrilateral.
		 */
	

		item :>> faces [5];
		item :>> ff : Triangle {        
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;            
        }
		item :>> rf : Triangle {        
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;            
        }

		item :>> edges [18];

		item :>> vertices;

		/* Bind face edges to specific edges */
		binding [1] bind [0..1] tf.edges = [0..1] bsre;

		/* Bind edge vertices to specific vertices */
		binding [1] bind [0..1] tfe.vertices = [0..1] bfrv;
		binding [1] bind [0..1] tre.vertices = [0..1] bfrv;
	}

	item def RightTriangularPrism :> TriangularPrism {
		doc
		/*
		 * A RightTriangularPrism  a TriangularPrism with two right triangluar sides,
		 * with given length, width, and height.
		 */
	 
		attribute :>> length [1];
		attribute :>> width [1];
		attribute :>> height [1];

		item :>> tf  : Rectangle;
		item :>> bf  : Rectangle;
		item :>> ff : RightTriangle {
			attribute :>> length = RightTriangularPrism::length;
			attribute :>> width = RightTriangularPrism::width;
		}
		item :>> rf : RightTriangle {
			attribute :>> length = ff.length;
			attribute :>> width = rf.width;
		}
		item :>> slf : Rectangle;
		item :>> srf : Rectangle;

		item :>> tfe  { attribute :>> length = ff.hypotenuse.length; }
		item :>> tre  { attribute :>> length = tfe.length; }
		item :>> tsle { attribute :>> length = height; }
		item :>> bfe  { attribute :>> length = RightTriangularPrism::length; }
		item :>> bre  { attribute :>> length = RightTriangularPrism::length; }
		item :>> bsle { attribute :>> length = height; }
		item :>> bsre { attribute :>> length = height; }
		item :>> ufle { attribute :>> length = width;  } 
		item :>> urle { attribute :>> length = width; }
	}
	alias Wedge for RightTriangularPrism;

	item def Cuboid :> CuboidOrTriangularPrism {
		doc
		/*
		 * A Cuboid is a Polyhedron with six sides, all quadrilateral.
		 */	

		item :>> faces [6];
		item :>> ff : Quadrilateral {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }
		item :>> rf : Quadrilateral {        
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;            
        }

		item :>> edges [24];

		item :>> vertices;

		/* Bind face edges to specific edges */
		binding [1] bind [0..1] tf.edges = [0..1] tsre;
		binding [1] bind [0..1] ff.edges = [0..1] ufre;
		binding [1] bind [0..1] rf.edges = [0..1] urre;

		binding [1] bind [0..1] srf.edges = [0..1] tsre;
		binding [1] bind [0..1] srf.edges = [0..1] bsre;
		binding [1] bind [0..1] srf.edges = [0..1] ufre;
		binding [1] bind [0..1] srf.edges = [0..1] urre;

		/* Bind edge vertices to specific vertices */
		binding [1] bind [0..1] tfe.vertices = [0..1] tfrv;
		binding [1] bind [0..1] tre.vertices = [0..1] trrv;
		binding [1] bind [0..1] tsre.vertices = [0..1] tfrv;
		binding [1] bind [0..1] tsre.vertices = [0..1] trrv;

		binding [1] bind [0..1] ufre.vertices = [0..1] tfrv;
		binding [1] bind [0..1] ufre.vertices = [0..1] bfrv;
		binding [1] bind [0..1] urre.vertices = [0..1] trrv;
		binding [1] bind [0..1] urre.vertices = [0..1] brrv;

		/* Meeting edges */
		connection :MatesWith connect [1] tsre to [1] tsre;
		connection :MatesWith connect [1] ufre to [1] ufre;
		connection :MatesWith connect [1] urre to [1] urre;
		connection :MatesWith connect [1] bsre to [1] bsre;

		/* Meeting vertices  */
		connection :MatesWith connect [2] tfrv to [2] tfrv;
		connection :MatesWith connect [2] trrv to [2] trrv;
	}

	item def RectangularCuboid :> Cuboid {
		doc
		/*
		 * A RectangularCuboid is a Cuboid with all Rectangular sides.
		 */
	
		attribute :>> length [1];
		attribute :>> width [1];
		attribute :>> height [1];
	
		item :>> tf  : Rectangle { attribute :>> length = RectangularCuboid::length;
								   attribute :>> width	= RectangularCuboid::height; }
		item :>> bf  : Rectangle { attribute :>> length = RectangularCuboid::length;
								   attribute :>> width	= RectangularCuboid::height; }
		item :>> ff  : Rectangle { attribute :>> length = RectangularCuboid::length;
								   attribute :>> width	= RectangularCuboid::width; }
		item :>> rf  : Rectangle { attribute :>> length = RectangularCuboid::length;
								   attribute :>> width	= RectangularCuboid::width; }
		item :>> slf : Rectangle { attribute :>> length = RectangularCuboid::height;
								   attribute :>> width	= RectangularCuboid::width; }
		item :>> srf : Rectangle { attribute :>> length = RectangularCuboid::height;
								   attribute :>> width	= RectangularCuboid::width; }
	}
	alias Box for RectangularCuboid;

	item def Pyramid :> Polyhedron {
		doc
		/*
		 * A Pyramid is a Polyhedron with the sides of a polygon (base) forming the bases of triangles
		 * that join at an apex point.	Its height is the perpendicular distance from the base to the apex,
		 * and its offsets are between this perpendicular at the base and the center of the base.
		 */	 

		attribute :>> height [1];
		attribute :>> xoffset;
		attribute :>> yoffset;

		item :>> faces;
		item base [1] :> faces;
		item wall : Triangle :> faces {        
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;            
        }
		attribute wallNumber : Positive = size(wall);

		assert constraint { size(faces) == wallNumber + 1 }
		assert constraint { size(wall) == size(base.edges) }

		item :>> edges;

		assert constraint { size(edges) == wallNumber * 4 }

		item :>> vertices;
		item apex :> vertices = wall.apex;

		assert constraint { size(apex) == wallNumber }

		/* Base to wall and wall to wall edge mating. */
		assert constraint { (1..wallNumber)->forAll {in i;
					includes(wall#(i).base.matingOccurrences,
							 Pyramid::base.edges#(i)) and
					includes((wall#(i).edges#(3) as Item).matingOccurrences,
							 wall#(if i==wallNumber ? 1 else i+1).edges#(2)) } }

		/* Meeting apices. */
		connection :MatesWith connect [wallNumber] apex to [wallNumber] apex;
	}

	item def Tetrahedron :> Pyramid {
		doc
		/*
		 * A Tetrahedron is Pyramid with a triangular base.
		 */
	
		attribute :>> baseLength [1];
		attribute :>> baseWidth [1];

		item :>> base : Triangle {
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;            
			attribute :>> length = Tetrahedron::baseLength;
			attribute :>> width  = Tetrahedron::baseWidth;
		}
	}

	item def RectangularPyramid :> Pyramid {
		doc
		/*
		 * A RectangularPyramid is Pyramid with a rectangular base.
		 */	

		attribute :>> baseLength [1];
		attribute :>> baseWidth [1];

		item :>> base : Rectangle {
            ref :>> Rectangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Rectangle::vertices, ConeOrCylinder::faces::vertices;            
			attribute :>> length = RectangularPyramid::baseLength;
			attribute :>> width = RectangularPyramid::baseWidth;
		}
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Curve'
semantic.unresolved_name 'length'
semantic.unresolved_name 'outerSpaceDimension'
semantic.unresolved_name 'Surface'
semantic.unresolved_name 'area'
semantic.unresolved_name 'outerSpaceDimension'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'length'
semantic.unresolved_name 'outerSpaceDimension'
semantic.unresolved_name 'StructuredSpaceObject::StructuredCurve'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'isClosed'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'StructuredSpaceObject::StructuredSurface'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'Shell::edges::innerSpaceDimension'
semantic.unresolved_name 'Ellipse::innerSpaceDimension'
semantic.unresolved_name 'Shell::edges::vertices'
semantic.unresolved_name 'Ellipse::vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Disc::shape::semiMajorAxis'
semantic.unresolved_name 'Disc::shape::semiMinorAxis'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'genus'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'isClosed'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'genus'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'revolvedCurve::isClosed'
semantic.unresolved_name 'Rectangle::isClosed'
semantic.unresolved_name 'height'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Disc::innerSpaceDimension'
semantic.unresolved_name 'faces::innerSpaceDimension'
semantic.unresolved_name 'Disc::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Disc::edges::innerSpaceDimension'
semantic.unresolved_name 'ConeOrCylinder::faces::edges::innerSpaceDimension'
semantic.unresolved_name 'Disc::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Disc::innerSpaceDimension'
semantic.unresolved_name 'faces::innerSpaceDimension'
semantic.unresolved_name 'Disc::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Disc::edges::innerSpaceDimension'
semantic.unresolved_name 'ConeOrCylinder::faces::edges::innerSpaceDimension'
semantic.unresolved_name 'Disc::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'Surface'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'genus'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'base::edges'
semantic.unresolved_name 'CircularDisc::edges'
semantic.unresolved_name 'num'
semantic.unresolved_name 'num'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'base::edges'
semantic.unresolved_name 'CircularDisc::edges'
semantic.unresolved_name 'af::edges'
semantic.unresolved_name 'CircularDisc::edges'
semantic.unresolved_name 'num'
semantic.unresolved_name 'num'
semantic.unresolved_name 'isClosed'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Polygon::innerSpaceDimension'
semantic.unresolved_name 'faces::innerSpaceDimension'
semantic.unresolved_name 'Polygon::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Polygon::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'outerSpaceDimension'
semantic.unresolved_name 'genus'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Polygon::edges'
semantic.unresolved_name 'faces::edges'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Polygon::edges'
semantic.unresolved_name 'faces::edges'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Triangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Triangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'Triangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Triangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Triangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Triangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'Triangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Triangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'Rectangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Rectangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Curve'
semantic.unresolved_name 'length'
semantic.unresolved_name 'outerSpaceDimension'
semantic.unresolved_name 'Surface'
semantic.unresolved_name 'area'
semantic.unresolved_name 'outerSpaceDimension'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'length'
semantic.unresolved_name 'outerSpaceDimension'
semantic.unresolved_name 'StructuredSpaceObject::StructuredCurve'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'isClosed'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'StructuredSpaceObject::StructuredSurface'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'Shell::edges::innerSpaceDimension'
semantic.unresolved_name 'Ellipse::innerSpaceDimension'
semantic.unresolved_name 'Shell::edges::vertices'
semantic.unresolved_name 'Ellipse::vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Disc::shape::semiMajorAxis'
semantic.unresolved_name 'Disc::shape::semiMinorAxis'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'genus'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'isClosed'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'genus'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'revolvedCurve::isClosed'
semantic.unresolved_name 'Rectangle::isClosed'
semantic.unresolved_name 'height'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Disc::innerSpaceDimension'
semantic.unresolved_name 'faces::innerSpaceDimension'
semantic.unresolved_name 'Disc::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Disc::edges::innerSpaceDimension'
semantic.unresolved_name 'ConeOrCylinder::faces::edges::innerSpaceDimension'
semantic.unresolved_name 'Disc::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Disc::innerSpaceDimension'
semantic.unresolved_name 'faces::innerSpaceDimension'
semantic.unresolved_name 'Disc::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Disc::edges::innerSpaceDimension'
semantic.unresolved_name 'ConeOrCylinder::faces::edges::innerSpaceDimension'
semantic.unresolved_name 'Disc::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'Surface'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'genus'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'base::edges'
semantic.unresolved_name 'CircularDisc::edges'
semantic.unresolved_name 'num'
semantic.unresolved_name 'num'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'base::edges'
semantic.unresolved_name 'CircularDisc::edges'
semantic.unresolved_name 'af::edges'
semantic.unresolved_name 'CircularDisc::edges'
semantic.unresolved_name 'num'
semantic.unresolved_name 'num'
semantic.unresolved_name 'isClosed'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Polygon::innerSpaceDimension'
semantic.unresolved_name 'faces::innerSpaceDimension'
semantic.unresolved_name 'Polygon::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Polygon::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'outerSpaceDimension'
semantic.unresolved_name 'genus'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Polygon::edges'
semantic.unresolved_name 'faces::edges'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Polygon::edges'
semantic.unresolved_name 'faces::edges'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Triangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Triangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'Triangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Triangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'Quadrilateral::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Quadrilateral::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'Triangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Triangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'vertices'
semantic.unresolved_name 'MatesWith'
semantic.unresolved_name 'Triangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Triangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'Rectangle::edges'
semantic.unresolved_name 'ConeOrCylinder::faces::edges'
semantic.unresolved_name 'Rectangle::vertices'
semantic.unresolved_name 'ConeOrCylinder::faces::vertices'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,UnrestrictedName,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,Ampersand,Ident,LtEq,DecimalValue,CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwAbstract,KwItem,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,OpenSquare,Star,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,EqEq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,Arrow,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,Ident,BangEq,Ident,KwAnd,
Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,CloseCurly,CloseCurly,CloseCurly,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,KwDefault,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,KwDefault,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwItem,ColonGtGt,Ident,OpenCurly,
KwAttribute,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,ColonColon,Ident,Star,Ident,ColonColon,Ident,Star,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAssert,KwConstraint,OpenCurly,OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,
Ident,Hash,OpenParen,Ident,CloseParen,Dot,Ident,Arrow,Ident,OpenParen,OpenParen,Ident,Hash,OpenParen,OpenParen,DecimalValue,Star,Ident,CloseParen,Minus,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,Star,Ident,CloseParen,CloseParen,CloseParen,KwAnd,
Ident,OpenParen,OpenParen,Ident,Hash,OpenParen,Ident,CloseParen,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,KwAs,Ident,CloseParen,Dot,Ident,Comma,
Ident,Hash,OpenParen,KwIf,Ident,EqEq,Ident,OpenParen,Ident,CloseParen,Question,DecimalValue,KwElse,Ident,Plus,DecimalValue,CloseParen,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,CloseCurly,CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Eq,OpenParen,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Eq,OpenParen,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Eq,OpenParen,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Slash,DecimalValue,Semicolon,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,Ident,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,ColonColon,Ident,Caret,DecimalValue,Plus,Ident,ColonColon,Ident,Caret,DecimalValue,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Eq,OpenParen,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Eq,OpenParen,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Eq,OpenParen,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Eq,OpenParen,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAbstract,KwItem,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwRef,KwItem,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,ColonGtGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,OpenParen,KwIf,Ident,OpenParen,Ident,CloseParen,Question,DecimalValue,KwElse,DecimalValue,CloseParen,KwAnd,
Ident,OpenParen,Ident,CloseParen,EqEq,OpenParen,KwIf,Ident,OpenParen,Ident,CloseParen,Question,DecimalValue,KwElse,DecimalValue,CloseParen,CloseCurly,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseCurly,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Semicolon,
RegularComment,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,ColonGtGt,Ident,Semicolon,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAssert,KwConstraint,OpenCurly,Ident,CloseAngle,DecimalValue,KwOr,Ident,CloseAngle,DecimalValue,CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAssert,KwConstraint,OpenCurly,Ident,CloseAngle,DecimalValue,KwOr,Ident,CloseAngle,DecimalValue,CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwIf,Ident,OpenParen,Ident,CloseParen,CloseAngle,DecimalValue,Question,DecimalValue,KwElse,DecimalValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,KwItem,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,KwItem,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,DecimalValue,KwOr,Ident,OpenParen,Ident,CloseParen,EqEq,DecimalValue,CloseCurly,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,OpenParen,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,OpenParen,Ident,CloseParen,CloseParen,KwAnd,
OpenParen,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseParen,KwAnd,
OpenParen,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseParen,CloseCurly,
KwItem,ColonGtGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,OpenParen,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseParen,CloseCurly,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Semicolon,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Semicolon,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Semicolon,
KwItem,ColonGtGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwItem,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,Plus,DecimalValue,CloseCurly,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,Dot,Ident,CloseParen,CloseCurly,
KwItem,ColonGtGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,Star,DecimalValue,CloseCurly,
KwItem,ColonGtGt,Ident,Semicolon,
KwItem,Ident,ColonGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,CloseCurly,
RegularComment,
KwAssert,KwConstraint,OpenCurly,OpenParen,DecimalValue,DotDot,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,
Ident,OpenParen,Ident,Hash,OpenParen,Ident,CloseParen,Dot,Ident,Dot,Ident,Comma,
Ident,ColonColon,Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,CloseParen,KwAnd,
Ident,OpenParen,OpenParen,Ident,Hash,OpenParen,Ident,CloseParen,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,KwAs,Ident,CloseParen,Dot,Ident,Comma,
Ident,Hash,OpenParen,KwIf,Ident,EqEq,Ident,Question,DecimalValue,KwElse,Ident,Plus,DecimalValue,CloseParen,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseParen,CloseCurly,CloseCurly,
RegularComment,
KwConnection,Colon,Ident,KwConnect,OpenSquare,Ident,CloseSquare,Ident,KwTo,OpenSquare,Ident,CloseSquare,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ShapeItems'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::Positive')
    (import_decl private 'ISQSpaceTime::*')
    (import_decl private 'ISQBase::*')
    (import_decl private 'SI::m')
    (import_decl private 'Occurrences::MatesWith')
    (import_decl private 'Objects::*')
    (import_decl private 'Items::Item')
    (import_decl private 'SequenceFunctions::equals')
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'SequenceFunctions::notEmpty')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'SequenceFunctions::includes')
    (import_decl private 'ControlFunctions::'if'')
    (import_decl private 'ControlFunctions::forAll')
    (import_decl private 'ControlFunctions::exists')
    (import_decl private 'Quantities::scalarQuantities')
    (item_def 'PlanarCurve' :> 'Curve'
      (documentation)
      (attribute_usage :>> 'length' multiplicity)
      (attribute_usage :>> 'outerSpaceDimension')
      (sysml_decl
        (result_expr_member)))
    (item_def 'PlanarSurface' :> 'Surface'
      (documentation)
      (attribute_usage :>> 'area' multiplicity)
      (attribute_usage :>> 'outerSpaceDimension' value)
      (item_usage :>> 'shape' : 'PlanarCurve'))
    (item_def 'Line' :> 'PlanarCurve'
      (documentation)
      (attribute_usage :>> 'length' multiplicity)
      (attribute_usage :>> 'outerSpaceDimension' value))
    (item_def abstract 'Path' :> 'StructuredSpaceObject::StructuredCurve'
      (documentation)
      (item_usage :>> 'faces' multiplicity)
      (item_usage :>> 'edges' multiplicity
        (item_usage :>> 'vertices' multiplicity))
      (item_usage :>> 'vertices' multiplicity value)
      (sysml_decl
        (result_expr_member)))
    (attribute_usage 'semiMajorAxis' : 'LengthValue' :> 'scalarQuantities' multiplicity)
    (attribute_usage 'semiMinorAxis' : 'LengthValue' :> 'scalarQuantities' multiplicity)
    (attribute_usage 'xoffset' : 'LengthValue' :> 'scalarQuantities' multiplicity value)
    (attribute_usage 'yoffset' : 'LengthValue' :> 'scalarQuantities' multiplicity value)
    (attribute_usage 'baseLength' : 'LengthValue' :> 'scalarQuantities' multiplicity)
    (attribute_usage 'baseWidth' : 'LengthValue' :> 'scalarQuantities' multiplicity)
    (item_def 'ConicSection' :> 'Path', 'PlanarCurve'
      (documentation)
      (item_usage :>> 'edges' multiplicity)
      (item_usage :>> 'vertices' multiplicity))
    (item_def 'Ellipse' :> 'ConicSection'
      (documentation)
      (attribute_usage :>> 'semiMajorAxis' multiplicity)
      (attribute_usage :>> 'semiMinorAxis' multiplicity)
      (item_usage :>> 'edges' multiplicity))
    (item_def 'Circle' :> 'Ellipse'
      (documentation)
      (attribute_usage :>> 'radius' multiplicity)
      (attribute_usage :>> 'semiMajorAxis' multiplicity value)
      (attribute_usage :>> 'semiMinorAxis' multiplicity value)
      (item_usage :>> 'edges'
        (attribute_usage 'length' multiplicity value)))
    (item_def 'Parabola' :> 'ConicSection'
      (documentation)
      (attribute_usage 'focalDistance' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (item_usage :>> 'edges' multiplicity))
    (item_def 'Hyperbola' :> 'ConicSection'
      (documentation)
      (attribute_usage 'tranverseAxis' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (attribute_usage 'conjugateAxis' : 'LengthValue' :> 'scalarQuantities' multiplicity))
    (item_def 'Polygon' :> 'Path', 'PlanarCurve'
      (documentation)
      (item_usage :>> 'edges' : 'Line'
        (item_usage :>> 'vertices' multiplicity))
      (attribute_usage :>> 'isClosed' value)
      (sysml_decl
        (result_expr_member)))
    (item_def 'Triangle' :> 'Polygon'
      (documentation)
      (attribute_usage :>> 'length' multiplicity)
      (attribute_usage :>> 'width' multiplicity)
      (attribute_usage :>> 'xoffset' multiplicity)
      (item_usage :>> 'edges' multiplicity value)
      (item_usage 'base' multiplicity
        (default_ref_usage 'length' value))
      (item_usage 'e2' multiplicity)
      (item_usage 'e3' multiplicity)
      (item_usage :>> 'vertices' multiplicity)
      (item_usage 'v12' multiplicity ordered value)
      (item_usage 'apex' multiplicity ordered value)
      (item_usage 'v31' multiplicity ordered value))
    (item_def 'RightTriangle' :> 'Triangle'
      (documentation)
      (attribute_usage :>> 'xoffset' value)
      (item_usage :>> 'e2'
        (attribute_usage :>> 'length' value))
      (item_usage 'hypotenuse' :>> 'e3'
        (attribute_usage :>> 'length' value)))
    (item_def 'Quadrilateral' :> 'Polygon'
      (documentation)
      (item_usage :>> 'edges' multiplicity value)
      (item_usage 'e1' multiplicity)
      (item_usage 'e2' multiplicity)
      (item_usage 'e3' multiplicity)
      (item_usage 'e4' multiplicity)
      (item_usage :>> 'vertices' multiplicity)
      (item_usage 'v12' multiplicity ordered value)
      (item_usage 'v23' multiplicity ordered value)
      (item_usage 'v34' multiplicity ordered value)
      (item_usage 'v41' multiplicity ordered value))
    (item_def 'Rectangle' :> 'Quadrilateral'
      (documentation)
      (attribute_usage :>> 'length' multiplicity)
      (attribute_usage :>> 'width' multiplicity)
      (item_usage :>> 'e1'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'e2'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'e3'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'e4'
        (attribute_usage :>> 'length' value)))
    (item_def abstract 'Shell' :> 'StructuredSpaceObject::StructuredSurface'
      (documentation))
    (item_def 'Disc' :> 'Shell', 'PlanarSurface'
      (documentation)
      (attribute_usage :>> 'semiMajorAxis' multiplicity)
      (attribute_usage :>> 'semiMinorAxis' multiplicity)
      (item_usage :>> 'shape' : 'Ellipse' multiplicity
        (attribute_usage :>> 'semiMajorAxis' value)
        (attribute_usage :>> 'semiMinorAxis' value))
      (item_usage :>> 'faces' : 'PlanarSurface' multiplicity
        (item_usage :>> 'edges' multiplicity))
      (item_usage :>> 'edges' : 'Ellipse' multiplicity value
        (attribute_usage :>> 'Shell::edges::innerSpaceDimension', 'Ellipse::innerSpaceDimension')
        (item_usage ref :>> 'Shell::edges::vertices', 'Ellipse::vertices'))
      (item_usage :>> 'vertices' multiplicity))
    (item_def 'CircularDisc' :> 'Disc'
      (documentation)
      (attribute_usage :>> 'radius' multiplicity)
      (attribute_usage :>> 'semiMajorAxis' multiplicity value)
      (attribute_usage :>> 'semiMinorAxis' multiplicity value)
      (item_usage :>> 'shape' : 'Circle'
        (attribute_usage :>> 'Disc::shape::semiMajorAxis', 'Circle::semiMajorAxis')
        (attribute_usage :>> 'Disc::shape::semiMinorAxis', 'Circle::semiMinorAxis'))
      (item_usage :>> 'edges' : 'Circle'))
    (item_def 'ConicSurface' :> 'Shell'
      (documentation)
      (item_usage :>> 'faces' multiplicity)
      (item_usage :>> 'edges' multiplicity)
      (item_usage :>> 'vertices' multiplicity)
      (attribute_usage :>> 'genus' value))
    (item_def 'Ellipsoid' :> 'ConicSurface'
      (documentation)
      (attribute_usage 'semiAxis1' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (attribute_usage 'semiAxis2' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (attribute_usage 'semiAxis3' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (item_usage :>> 'faces' multiplicity))
    (item_def 'Sphere' :> 'Ellipsoid'
      (documentation)
      (attribute_usage :>> 'radius' multiplicity)
      (attribute_usage :>> 'semiAxis1' multiplicity value)
      (attribute_usage :>> 'semiAxis2' multiplicity value)
      (attribute_usage :>> 'semiAxis3' multiplicity value))
    (item_def 'Paraboloid' :> 'ConicSurface'
      (documentation)
      (attribute_usage 'focalDistance' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (item_usage :>> 'faces' multiplicity))
    (item_def 'Hyperboloid' :> 'ConicSurface'
      (documentation)
      (attribute_usage 'transverseAxis' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (attribute_usage 'conjugateAxis' : 'LengthValue' :> 'scalarQuantities' multiplicity))
    (item_def 'Toroid' :> 'Shell'
      (documentation)
      (attribute_usage 'revolutionRadius' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (item_usage 'revolvedCurve' : 'PlanarCurve' multiplicity
        (attribute_usage :>> 'isClosed' value))
      (item_usage :>> 'faces' multiplicity)
      (item_usage :>> 'edges' multiplicity)
      (item_usage :>> 'vertices' multiplicity)
      (attribute_usage :>> 'genus' value))
    (item_def 'Torus' :> 'Toroid'
      (documentation)
      (attribute_usage 'majorRadius' :>> 'revolutionRadius')
      (attribute_usage 'minorRadius' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (item_usage :>> 'revolvedCurve' : 'Circle' multiplicity
        (attribute_usage :>> 'radius' value)))
    (item_def 'RectangularToroid' :> 'Toroid'
      (documentation)
      (attribute_usage 'rectangleLength' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (attribute_usage 'rectangleWidth' : 'LengthValue' :> 'scalarQuantities' multiplicity)
      (item_usage :>> 'revolvedCurve' : 'Rectangle' multiplicity
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value)
        (attribute_usage :>> 'revolvedCurve::isClosed', 'Rectangle::isClosed')))
    (item_def 'ConeOrCylinder' :> 'Shell'
      (documentation)
      (attribute_usage :>> 'semiMajorAxis' multiplicity)
      (attribute_usage :>> 'semiMinorAxis' multiplicity)
      (attribute_usage :>> 'height' multiplicity)
      (attribute_usage :>> 'xoffset' multiplicity)
      (attribute_usage :>> 'yoffset' multiplicity)
      (item_usage :>> 'faces' multiplicity)
      (item_usage 'base' : 'Disc' :> 'faces' multiplicity
        (attribute_usage :>> 'Disc::innerSpaceDimension', 'faces::innerSpaceDimension')
        (ref_usage ref :>> 'Disc::edges', 'ConeOrCylinder::faces::edges'
          (attribute_usage :>> 'Disc::edges::innerSpaceDimension', 'ConeOrCylinder::faces::edges::innerSpaceDimension'))
        (ref_usage ref :>> 'Disc::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage 'af' : 'Disc' :> 'faces' multiplicity
        (attribute_usage :>> 'Disc::innerSpaceDimension', 'faces::innerSpaceDimension')
        (ref_usage ref :>> 'Disc::edges', 'ConeOrCylinder::faces::edges'
          (attribute_usage :>> 'Disc::edges::innerSpaceDimension', 'ConeOrCylinder::faces::edges::innerSpaceDimension'))
        (ref_usage ref :>> 'Disc::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage 'cf' : 'Surface' :> 'faces' multiplicity)
      (item_usage :>> 'edges' multiplicity value)
      (item_usage 'be' :> 'edges' multiplicity
        (attribute_usage :>> 'semiMajorAxis' value)
        (attribute_usage :>> 'semiMinorAxis' value))
      (item_usage 'ae' :> 'edges' multiplicity
        (attribute_usage :>> 'semiMajorAxis' value)
        (attribute_usage :>> 'semiMinorAxis' value))
      (sysml_decl
        (result_expr_member))
      (item_usage :>> 'vertices' multiplicity value)
      (sysml_decl
        (result_expr_member))
      (comment)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (comment)
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (attribute_usage :>> 'genus' value))
    (item_def 'Cone' :> 'ConeOrCylinder'
      (documentation)
      (item_usage :>> 'faces' multiplicity)
      (item_usage 'apex' :>> 'vertices')
      (comment)
      (binding_connector multiplicity
        (connector_end)
        (connector_end)))
    (item_def 'EccentricCone' :> 'Cone'
      (documentation)
      (sysml_decl
        (result_expr_member)))
    (item_def 'CircularCone' :> 'Cone'
      (documentation)
      (attribute_usage :>> 'radius' multiplicity)
      (attribute_usage :>> 'semiMajorAxis' multiplicity value)
      (attribute_usage :>> 'semiMinorAxis' multiplicity value)
      (item_usage :>> 'base' : 'CircularDisc'
        (ref_usage ref :>> 'base::edges', 'CircularDisc::edges')))
    (item_def 'RightCircularCone' :> 'CircularCone'
      (documentation)
      (attribute_usage :>> 'xoffset'
        (attribute_usage :>> 'num' value))
      (attribute_usage :>> 'yoffset'
        (attribute_usage :>> 'num' value)))
    (item_def 'Cylinder' :> 'ConeOrCylinder'
      (documentation)
      (item_usage :>> 'af' multiplicity)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end)
        (documentation)))
    (item_def 'EccentricCylinder' :> 'Cylinder'
      (documentation)
      (sysml_decl
        (result_expr_member)))
    (item_def 'CircularCylinder' :> 'Cylinder'
      (documentation)
      (attribute_usage :>> 'radius' multiplicity)
      (attribute_usage :>> 'semiMajorAxis' multiplicity value)
      (attribute_usage :>> 'semiMinorAxis' multiplicity value)
      (item_usage :>> 'base' : 'CircularDisc'
        (ref_usage ref :>> 'base::edges', 'CircularDisc::edges'))
      (item_usage :>> 'af' : 'CircularDisc'
        (ref_usage ref :>> 'af::edges', 'CircularDisc::edges')))
    (item_def 'RightCircularCylinder' :> 'CircularCylinder'
      (documentation)
      (attribute_usage :>> 'xoffset'
        (attribute_usage :>> 'num' value))
      (attribute_usage :>> 'yoffset'
        (attribute_usage :>> 'num' value)))
    (item_def 'Polyhedron' :> 'Shell'
      (documentation)
      (attribute_usage :>> 'isClosed' value)
      (item_usage :>> 'faces' : 'Polygon' multiplicity
        (attribute_usage :>> 'Polygon::innerSpaceDimension', 'faces::innerSpaceDimension')
        (ref_usage ref :>> 'Polygon::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Polygon::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage :>> 'edges' value)
      (attribute_usage :>> 'outerSpaceDimension' value)
      (attribute_usage :>> 'genus' value))
    (item_def 'CuboidOrTriangularPrism' :> 'Polyhedron'
      (documentation)
      (item_usage :>> 'faces' multiplicity)
      (item_usage 'tf' : 'Quadrilateral' :> 'faces' multiplicity
        (ref_usage ref :>> 'Quadrilateral::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Quadrilateral::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage 'bf' : 'Quadrilateral' :> 'faces' multiplicity
        (ref_usage ref :>> 'Quadrilateral::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Quadrilateral::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage 'ff' : 'Polygon' :> 'faces' multiplicity
        (item_usage :>> 'Polygon::edges', 'faces::edges' multiplicity))
      (item_usage 'rf' : 'Polygon' :> 'faces' multiplicity
        (item_usage :>> 'Polygon::edges', 'faces::edges' multiplicity))
      (item_usage 'slf' : 'Quadrilateral' :> 'faces' multiplicity
        (ref_usage ref :>> 'Quadrilateral::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Quadrilateral::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage 'srf' : 'Quadrilateral' :> 'faces' multiplicity
        (ref_usage ref :>> 'Quadrilateral::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Quadrilateral::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage :>> 'edges')
      (sysml_decl
        (result_expr_member))
      (item_usage 'tfe' :> 'edges' multiplicity)
      (item_usage 'tre' :> 'edges' multiplicity)
      (item_usage 'tsle' :> 'edges' multiplicity)
      (item_usage 'tsre' :> 'edges' multiplicity)
      (item_usage 'bfe' :> 'edges' multiplicity)
      (item_usage 'bre' :> 'edges' multiplicity)
      (item_usage 'bsle' :> 'edges' multiplicity)
      (item_usage 'bsre' :> 'edges' multiplicity)
      (item_usage 'ufle' :> 'edges' multiplicity)
      (item_usage 'ufre' :> 'edges' multiplicity)
      (item_usage 'urle' :> 'edges' multiplicity)
      (item_usage 'urre' :> 'edges' multiplicity)
      (sysml_decl
        (result_expr_member))
      (item_usage :>> 'vertices')
      (sysml_decl
        (result_expr_member))
      (item_usage 'tflv' :> 'vertices' multiplicity)
      (item_usage 'tfrv' :> 'vertices' multiplicity)
      (item_usage 'trlv' :> 'vertices' multiplicity)
      (item_usage 'trrv' :> 'vertices' multiplicity)
      (item_usage 'bflv' :> 'vertices' multiplicity)
      (item_usage 'bfrv' :> 'vertices' multiplicity)
      (item_usage 'brlv' :> 'vertices' multiplicity)
      (item_usage 'brrv' :> 'vertices' multiplicity)
      (sysml_decl
        (result_expr_member))
      (comment)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (comment)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (comment)
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (comment)
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end)))
    (item_def 'TriangularPrism' :> 'CuboidOrTriangularPrism'
      (documentation)
      (item_usage :>> 'faces' multiplicity)
      (item_usage :>> 'ff' : 'Triangle'
        (ref_usage ref :>> 'Triangle::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Triangle::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage :>> 'rf' : 'Triangle'
        (ref_usage ref :>> 'Triangle::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Triangle::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage :>> 'edges' multiplicity)
      (item_usage :>> 'vertices')
      (comment)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (comment)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end)))
    (item_def 'RightTriangularPrism' :> 'TriangularPrism'
      (documentation)
      (attribute_usage :>> 'length' multiplicity)
      (attribute_usage :>> 'width' multiplicity)
      (attribute_usage :>> 'height' multiplicity)
      (item_usage :>> 'tf' : 'Rectangle')
      (item_usage :>> 'bf' : 'Rectangle')
      (item_usage :>> 'ff' : 'RightTriangle'
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value))
      (item_usage :>> 'rf' : 'RightTriangle'
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value))
      (item_usage :>> 'slf' : 'Rectangle')
      (item_usage :>> 'srf' : 'Rectangle')
      (item_usage :>> 'tfe'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'tre'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'tsle'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'bfe'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'bre'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'bsle'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'bsre'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'ufle'
        (attribute_usage :>> 'length' value))
      (item_usage :>> 'urle'
        (attribute_usage :>> 'length' value)))
    (alias_member 'Wedge' for 'RightTriangularPrism')
    (item_def 'Cuboid' :> 'CuboidOrTriangularPrism'
      (documentation)
      (item_usage :>> 'faces' multiplicity)
      (item_usage :>> 'ff' : 'Quadrilateral'
        (ref_usage ref :>> 'Quadrilateral::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Quadrilateral::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage :>> 'rf' : 'Quadrilateral'
        (ref_usage ref :>> 'Quadrilateral::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Quadrilateral::vertices', 'ConeOrCylinder::faces::vertices'))
      (item_usage :>> 'edges' multiplicity)
      (item_usage :>> 'vertices')
      (comment)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (comment)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (comment)
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (comment)
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end))
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end)))
    (item_def 'RectangularCuboid' :> 'Cuboid'
      (documentation)
      (attribute_usage :>> 'length' multiplicity)
      (attribute_usage :>> 'width' multiplicity)
      (attribute_usage :>> 'height' multiplicity)
      (item_usage :>> 'tf' : 'Rectangle'
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value))
      (item_usage :>> 'bf' : 'Rectangle'
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value))
      (item_usage :>> 'ff' : 'Rectangle'
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value))
      (item_usage :>> 'rf' : 'Rectangle'
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value))
      (item_usage :>> 'slf' : 'Rectangle'
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value))
      (item_usage :>> 'srf' : 'Rectangle'
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value)))
    (alias_member 'Box' for 'RectangularCuboid')
    (item_def 'Pyramid' :> 'Polyhedron'
      (documentation)
      (attribute_usage :>> 'height' multiplicity)
      (attribute_usage :>> 'xoffset')
      (attribute_usage :>> 'yoffset')
      (item_usage :>> 'faces')
      (item_usage 'base' :> 'faces' multiplicity)
      (item_usage 'wall' : 'Triangle' :> 'faces'
        (ref_usage ref :>> 'Triangle::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Triangle::vertices', 'ConeOrCylinder::faces::vertices'))
      (attribute_usage 'wallNumber' : 'Positive' value)
      (sysml_decl
        (result_expr_member))
      (sysml_decl
        (result_expr_member))
      (item_usage :>> 'edges')
      (sysml_decl
        (result_expr_member))
      (item_usage :>> 'vertices')
      (item_usage 'apex' :> 'vertices' value)
      (sysml_decl
        (result_expr_member))
      (comment)
      (sysml_decl
        (result_expr_member))
      (comment)
      (connection_usage 'MatesWith'
        (connector_end)
        (connector_end)))
    (item_def 'Tetrahedron' :> 'Pyramid'
      (documentation)
      (attribute_usage :>> 'baseLength' multiplicity)
      (attribute_usage :>> 'baseWidth' multiplicity)
      (item_usage :>> 'base' : 'Triangle'
        (ref_usage ref :>> 'Triangle::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Triangle::vertices', 'ConeOrCylinder::faces::vertices')
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value)))
    (item_def 'RectangularPyramid' :> 'Pyramid'
      (documentation)
      (attribute_usage :>> 'baseLength' multiplicity)
      (attribute_usage :>> 'baseWidth' multiplicity)
      (item_usage :>> 'base' : 'Rectangle'
        (ref_usage ref :>> 'Rectangle::edges', 'ConeOrCylinder::faces::edges')
        (ref_usage ref :>> 'Rectangle::vertices', 'ConeOrCylinder::faces::vertices')
        (attribute_usage :>> 'length' value)
        (attribute_usage :>> 'width' value)))))
~~~
# FORMAT
~~~sysml
standard library package ShapeItems {
    doc /*
	 * This package provides a model of items that represent basic geometric shapes. 
	 */

    private import ScalarValues::Boolean;
    private import ScalarValues::Positive;
    private import ISQSpaceTime::*;
    private import ISQBase::*;
    private import SI::m;
    private import Occurrences::MatesWith;
    private import Objects::*;
    private import Items::Item;
    private import SequenceFunctions::equals;
    private import SequenceFunctions::isEmpty;
    private import SequenceFunctions::notEmpty;
    private import SequenceFunctions::size;
    private import SequenceFunctions::includes;
    private import ControlFunctions::'if';
    private import ControlFunctions::forAll;
    private import ControlFunctions::exists;
    private import Quantities::scalarQuantities;

    item def PlanarCurve :> Curve {
        doc /*
		 * A PlanarCurve is a Curve with a given length embeddable in a plane.
		 */

        attribute :>> length [1];

        attribute :>> outerSpaceDimension;
        assert constraint {
            = notEmpty(outerSpaceDimension) & outerSpaceDimension <= 2;
        }
    }

    item def PlanarSurface :> Surface {
        doc /*
		 * A PlanarSurface is a flat Surface with a given area.
		 */

        attribute :>> area [1];
        attribute :>> outerSpaceDimension = 2;

        item :>> shape : PlanarCurve;
    }

    item def Line :> PlanarCurve {
        doc /*
		 * A Line is a Curve that is a straight line of a given length.
		 */

        attribute :>> length [1];
        attribute :>> outerSpaceDimension = 1;
    }

    abstract item def Path :> StructuredSpaceObject::StructuredCurve {
        doc /*
		 * Path is the most general structured Curve.
		 */

        item :>> faces [0];
        item :>> edges [1..*] {
            item :>> vertices [0..2];
        }
        item :>> vertices [*] = edges.vertices;

        assert constraint {
            = isClosed == vertices->forAll {in p1 : Point;
					vertices->exists{p2 : Point; p1 != p2 and
							 includes(p1.matingOccurrences, p2) } };
        }
    }

    attribute semiMajorAxis : LengthValue :> scalarQuantities [0..*];
    attribute semiMinorAxis : LengthValue :> scalarQuantities [0..*];
    attribute xoffset : LengthValue :> scalarQuantities [0..*] default = 0 [m];
    attribute yoffset : LengthValue :> scalarQuantities [0..*] default = 0 [m];
    attribute baseLength : LengthValue :> scalarQuantities [0..*];
    attribute baseWidth : LengthValue :> scalarQuantities [0..*];

    item def ConicSection :> Path, PlanarCurve {
        doc /*
		 * A ConicSection is a closed PlanarCurve, possibly disconnected, see Hyperbola.
		 */

        item :>> edges [1..2];

        item :>> vertices [0];
    }

    item def Ellipse :> ConicSection {
        doc /*
		 * An Ellipse is a ConicSection in the shape of an ellipse of a given semiaxes.
		 */

        attribute :>> semiMajorAxis [1];
        attribute :>> semiMinorAxis [1];

        item :>> edges [1];
    }

    item def Circle :> Ellipse {
        doc /*
		 * A Circle is an Ellipse with semiaxes equal to its radius.
		 */

        attribute :>> radius [1];
        attribute :>> semiMajorAxis [1] = radius;
        attribute :>> semiMinorAxis [1] = radius;

        item :>> edges {
            attribute length [1] = Circle::radius * TrigFunctions::pi * 2;
        }
    }

    item def Parabola :> ConicSection {
        doc /*
		 * A Parabola is a ConicSection in the shape of a parabola of a given focal length.
		 */

        attribute focalDistance : LengthValue :> scalarQuantities [1];

        item :>> edges [1];
    }

    item def Hyperbola :> ConicSection {
        doc /*
		 * A Hyperbola is a ConicSection in the shape of a hyperbola with given axes.
		 */

        attribute tranverseAxis : LengthValue :> scalarQuantities [1];
        attribute conjugateAxis : LengthValue :> scalarQuantities [1];
    }

    item def Polygon :> Path, PlanarCurve {
        doc /*
		 * A Polygon is a closed planar Path with straight edges.
		 */

        item :>> edges : Line {
            item :>> vertices [2];
        }

        attribute :>> isClosed = true;

        assert constraint {
            = (1 .. size(edges))->forAll {in i;
					edges#(i).vertices->equals((vertices#((2*i)-1), vertices#(2*i))) and  
					includes((edges#(i).vertices#(2) as Item).matingOccurrences,
						 edges#(if i==size(edges) ? 1 else i+1).vertices#(1)) };
        }
    }

    item def Triangle :> Polygon {
        doc /*
		 * A Triangle is three-sided Polygon  with given length (base), width (perpendicular distance
		 * from base to apex), and offset of this perpendicular at the base from the center of the base.
		 */

        attribute :>> length [1];
        attribute :>> width [1];
        attribute :>> xoffset [1];

        item :>> edges [3] = (base, e2, e3);
        item base [1] {
            length = Triangle::length;
        }
        item e2 [1];
        item e3 [1];

        item :>> vertices [6];
        item v12 [2] ordered = (vertices#(2), vertices#(3));
        item apex [2] ordered = (vertices#(4), vertices#(5));
        item v31 [2] ordered = (vertices#(6), vertices#(1));
    }

    item def RightTriangle :> Triangle {
        doc /*
		 * A RightTriangle is a Triangle with sides opposite the hypotenuse at right angles.
		 */

        attribute :>> xoffset = length / 2;

        item :>> e2 {
            attribute :>> length = Triangle::width;
        }

        item hypotenuse :>> e3 {
            attribute :>> length = ( Triangle::length^2 + Triangle::width^2 );
        }
    }

    item def Quadrilateral :> Polygon {
        doc /*
		 * A Quadrilateral is a four-sided Polygon.
		 */

        item :>> edges [4] = (e1, e2, e3, e4);
        item e1 [1];
        item e2 [1];
        item e3 [1];
        item e4 [1];

        item :>> vertices [8];
        item v12 [2] ordered = (vertices#(2), vertices#(3));
        item v23 [2] ordered = (vertices#(4), vertices#(5));
        item v34 [2] ordered = (vertices#(6), vertices#(7));
        item v41 [2] ordered = (vertices#(6), vertices#(1));
    }

    item def Rectangle :> Quadrilateral {
        doc /*
		 * A Rectangle is a Quadrilateral four right angles and given length and width.
		 */

        attribute :>> length [1];
        attribute :>> width [1];

        item :>> e1 {
            attribute :>> length = Rectangle::length;
        }
        item :>> e2 {
            attribute :>> length = Rectangle::width;
        }
        item :>> e3 {
            attribute :>> length = e1.length;
        }
        item :>> e4 {
            attribute :>> length = e2.length;
        }
    }

    abstract item def Shell :> StructuredSpaceObject::StructuredSurface {
        doc /*
		 * Shell is the most general structured Surface.
		 */
    }

    item def Disc :> Shell, PlanarSurface {
        doc /*
		 * A Disc is a Shell bound by an Ellipse.
		 */

        attribute :>> semiMajorAxis [1];
        attribute :>> semiMinorAxis [1];

        item :>> shape : Ellipse [1] {
            attribute :>> semiMajorAxis = Disc::semiMajorAxis;
            attribute :>> semiMinorAxis = Disc::semiMinorAxis;
        }

        item :>> faces : PlanarSurface [1] {
            item :>> edges [1];
        }
        item :>> edges : Ellipse [1] = shape {
            attribute :>> Shell::edges::innerSpaceDimension, Ellipse::innerSpaceDimension;
            ref item :>> Shell::edges::vertices, Ellipse::vertices;
        }
        item :>> vertices [0];
    }

    item def CircularDisc :> Disc {
        doc /*
		 * A CircularDisc is a Disc bound by a Circle.
		 */

        attribute :>> radius [1];
        attribute :>> semiMajorAxis [1] = radius;
        attribute :>> semiMinorAxis [1] = radius;

        item :>> shape : Circle {
            attribute :>> Disc::shape::semiMajorAxis, Circle::semiMajorAxis;
            attribute :>> Disc::shape::semiMinorAxis, Circle::semiMinorAxis;
        }
        item :>> edges : Circle;
    }

    item def ConicSurface :> Shell {
        doc /*
		 * A ConicSurface is a Surface that has ConicSection cross-sections.
		 */

        item :>> faces [1..2];
        item :>> edges [0];
        item :>> vertices [0];

        attribute :>> genus = 0;
    }

    item def Ellipsoid :> ConicSurface {
        doc /*
		 * An Ellipsoid is a ConicSurface with only elliptical cross-sections.
		 */

        attribute semiAxis1 : LengthValue :> scalarQuantities [1];
        attribute semiAxis2 : LengthValue :> scalarQuantities [1];
        attribute semiAxis3 : LengthValue :> scalarQuantities [1];

        item :>> faces [1];
    }

    item def Sphere :> Ellipsoid {
        doc /*
		 * A Sphere is an Ellipsoid with all the same semiaxes.
		 */

        attribute :>> radius [1];
        attribute :>> semiAxis1 [1] = radius;
        attribute :>> semiAxis2 [1] = radius;
        attribute :>> semiAxis3 [1] = radius;
    }

    item def Paraboloid :> ConicSurface {
        doc /*
		 * A Paraboloid is a ConicSurface with only parabolic cross-sections.
		 */

        attribute focalDistance : LengthValue :> scalarQuantities [1];

        item :>> faces [1];
    }

    item def Hyperboloid :> ConicSurface {
        doc /*
		 * A Hyperboloid is a ConicSurface with only hyperbolic cross-sections.
		 */

        attribute transverseAxis : LengthValue :> scalarQuantities [1];
        attribute conjugateAxis : LengthValue :> scalarQuantities [1];
    }

    item def Toroid :> Shell {
        doc /*
		 * A Toroid is a surface generated from revolving a planar closed curve about an line coplanar
		 * with the curve. It is single sided with one hole.
		 */

        attribute revolutionRadius : LengthValue :> scalarQuantities [1];

        item revolvedCurve : PlanarCurve [1] {
            attribute :>> isClosed = true;
        }

        item :>> faces [1];
        item :>> edges [0];
        item :>> vertices [0];

        attribute :>> genus = 1;
    }

    item def Torus :> Toroid {
        doc /*
		 * A Torus is a revolution of a Circle.
		 */

        attribute majorRadius :>> revolutionRadius;
        attribute minorRadius : LengthValue :> scalarQuantities [1];

        item :>> revolvedCurve : Circle [1] {
            attribute :>> radius = minorRadius;
        }
    }

    item def RectangularToroid :> Toroid {
        doc /*
		 * A RectangularToroid is a revolution of a Rectangle.
		 */

        attribute rectangleLength : LengthValue :> scalarQuantities [1];
        attribute rectangleWidth : LengthValue :> scalarQuantities [1];

        item :>> revolvedCurve : Rectangle [1] {
            attribute :>> length = rectangleLength;
            attribute :>> width = rectangleWidth;
            attribute :>> revolvedCurve::isClosed, Rectangle::isClosed;
        }
    }

    item def ConeOrCylinder :> Shell {
        doc /*
		 * A ConeOrCylinder is Shell that a Cone or a Cylinder with a given elliptical base,
		 * height, width (perpendicular distance from the base to the center of the top side or vertex),
		 * and offsets of this perpendicular at the base from the center of the base.
		 */

        attribute :>> semiMajorAxis [1];
        attribute :>> semiMinorAxis [1];
        attribute :>> height [1];

        attribute :>> xoffset [1];
        attribute :>> yoffset [1];

        item :>> faces [2..3];
        item base : Disc :> faces [1] {
            attribute :>> Disc::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Disc::edges, ConeOrCylinder::faces::edges {
                attribute :>> Disc::edges::innerSpaceDimension, ConeOrCylinder::faces::edges::innerSpaceDimension;
            }
            ref :>> Disc::vertices, ConeOrCylinder::faces::vertices;
        }
        item af : Disc :> faces [0..1] {
            attribute :>> Disc::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Disc::edges, ConeOrCylinder::faces::edges {
                attribute :>> Disc::edges::innerSpaceDimension, ConeOrCylinder::faces::edges::innerSpaceDimension;
            }
            ref :>> Disc::vertices, ConeOrCylinder::faces::vertices;
        }
        item cf : Surface :> faces [1];

        item :>> edges [2..4] = faces.edges;
        item be :> edges [2] {
            attribute :>> semiMajorAxis = ConeOrCylinder::semiMajorAxis;
            attribute :>> semiMinorAxis = ConeOrCylinder::semiMinorAxis;
        }
        item ae :> edges [0..2] {
            attribute :>> semiMajorAxis = be.semiMajorAxis;
            attribute :>> semiMinorAxis = be.semiMinorAxis;
        }
        assert constraint {
            = size(ae) == (if isEmpty(af) ? 0 else 2) and size(edges) == (if isEmpty(af) ? 2 else 4);
        }

        item :>> vertices [0..1] = faces.vertices;
        assert constraint {
            = isEmpty(af) == notEmpty(vertices);
        }

        /* Bind face edges to specific edges */
        binding [1] bind [0..*] base.edges = [0..*] be;
        binding [1] bind [0..*] cf.edges = [0..*] be;

        /* Meeting edges */
        connection : MatesWith connect [1] be to [1] be;

        attribute :>> genus = 0;
    }

    item def Cone :> ConeOrCylinder {
        doc /*
		 * A Cone has one elliptical sides joined to a point by a curved side.
		 */

        item :>> faces [2];

        item apex :>> vertices;

        /* Bind face vertices to specific vertices */
        binding [1] bind [0..*] cf.vertices = [0..*] apex;
    }

    item def EccentricCone :> Cone {
        doc /*
		 * An EccentricCone is a Cone with least one positive offset.
		 */

        assert constraint {
            = xoffset > 0 or yoffset > 0;
        }
    }

    item def CircularCone :> Cone {
        doc /*
		 * A CircularCone is a Cone with a circular base.
		 */

        attribute :>> radius [1];
        attribute :>> semiMajorAxis [1] = radius;
        attribute :>> semiMinorAxis [1] = radius;

        item :>> base : CircularDisc {
            ref :>> base::edges, CircularDisc::edges;
        }
    }

    item def RightCircularCone :> CircularCone {
        doc /*
		 * A RightCircularCone is a CircularCone with zero offsets.
		 */

        attribute :>> xoffset {
            attribute :>> num = 0;
        }
        attribute :>> yoffset {
            attribute :>> num = 0;
        }
    }

    item def Cylinder :> ConeOrCylinder {
        doc /*
		 * A Cylinder has two elliptical sides joined by a curved side.
		 */

        item :>> af [1];

        binding [1] bind [0..*] cf.edges = [0..*] ae;

        connection : MatesWith connect [1] ae to [1] ae {
            doc /* Meeting edges */
        }
    }

    item def EccentricCylinder :> Cylinder {
        doc /*
	 * An EccentricCylinder is a Cylinder with least one positive offset.
	 */

        assert constraint {
            = xoffset > 0 or yoffset > 0;
        }
    }

    item def CircularCylinder :> Cylinder {
        doc /*
		 * A CircularCylinder is a Cylinder with two circular sides.
		 */

        attribute :>> radius [1];
        attribute :>> semiMajorAxis [1] = radius;
        attribute :>> semiMinorAxis [1] = radius;

        item :>> base : CircularDisc {
            ref :>> base::edges, CircularDisc::edges;
        }
        item :>> af : CircularDisc {
            ref :>> af::edges, CircularDisc::edges;
        }
    }

    item def RightCircularCylinder :> CircularCylinder {
        doc /*
		 * A RightCircularCylinder is a CircularCylinder with zero offsets.
		 */

        attribute :>> xoffset {
            attribute :>> num = 0;
        }
        attribute :>> yoffset {
            attribute :>> num = 0;
        }
    }

    item def Polyhedron :> Shell {
        doc /*
		 * A Polyhedron is a closed Shell with polygonal sides.
		 */

        attribute :>> isClosed = true;

        item :>> faces : Polygon [2..*] {
            attribute :>> Polygon::innerSpaceDimension, faces::innerSpaceDimension;
            ref :>> Polygon::edges, ConeOrCylinder::faces::edges;
            ref :>> Polygon::vertices, ConeOrCylinder::faces::vertices;
        }

        item :>> edges = faces.edges;

        attribute :>> outerSpaceDimension = if size(faces) > 2 ? 3 else 2;

        attribute :>> genus = 0;
    }

    item def CuboidOrTriangularPrism :> Polyhedron {
        doc /*
		 * A CuboidOrTriangularPrism is a Polyhedron that is either a Cuboid or TriangularPrism.
		 */

        item :>> faces [5..6];
        item tf : Quadrilateral :> faces [1] {
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;
        }
        item bf : Quadrilateral :> faces [1] {
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;
        }
        item ff : Polygon :> faces [1] {
            item :>> Polygon::edges, faces::edges [3..4];
        }
        item rf : Polygon :> faces [1] {
            item :>> Polygon::edges, faces::edges [3..4];
        }
        item slf : Quadrilateral :> faces [1] {
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;
        }
        item srf : Quadrilateral :> faces [0..1] {
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;
        }

        item :>> edges;
        assert constraint {
            = size(edges) == 18 or size(edges) == 24;
        }

        item tfe :> edges [2];
        item tre :> edges [2];
        item tsle :> edges [2];
        item tsre :> edges [0..2];
        item bfe :> edges [2];
        item bre :> edges [2];
        item bsle :> edges [2];
        item bsre :> edges [2];
        item ufle :> edges [2];
        item ufre :> edges [0..2];
        item urle :> edges [2];
        item urre :> edges [0..2];

        assert constraint {
            = (isEmpty(srf) implies isEmpty(tsre)) and (isEmpty(tsre) == isEmpty(ufre)) and (isEmpty(ufre) == isEmpty(urre));
        }

        item :>> vertices;
        assert constraint {
            = size(vertices) == size(edges);
        }

        item tflv :> vertices [3];
        item tfrv :> vertices [0..3];
        item trlv :> vertices [3];
        item trrv :> vertices [0..3];
        item bflv :> vertices [3];
        item bfrv :> vertices [3];
        item brlv :> vertices [3];
        item brrv :> vertices [3];

        assert constraint {
            = (isEmpty(tfrv) == isEmpty(trrv));
        }

        /* Bind face edges to specific edges */
        binding [1] bind [0..1] tf.edges = [0..1] tfe;
        binding [1] bind [0..1] tf.edges = [0..1] tre;
        binding [1] bind [0..1] tf.edges = [0..1] tsle;
        binding [1] bind [0..1] bf.edges = [0..1] bfe;
        binding [1] bind [0..1] bf.edges = [0..1] bre;
        binding [1] bind [0..1] bf.edges = [0..1] bsle;
        binding [1] bind [0..1] bf.edges = [0..1] bsre;

        binding [1] bind [0..1] ff.edges = [0..1] tfe;
        binding [1] bind [0..1] ff.edges = [0..1] bfe;
        binding [1] bind [0..1] ff.edges = [0..1] ufle;

        binding [1] bind [0..1] rf.edges = [0..1] tre;
        binding [1] bind [0..1] rf.edges = [0..1] bre;
        binding [1] bind [0..1] rf.edges = [0..1] urle;

        /* Bind edge vertices to specific vertices */
        binding [1] bind [0..1] tfe.vertices = [0..1] tflv;
        binding [1] bind [0..1] tre.vertices = [0..1] trlv;
        binding [1] bind [0..1] tsle.vertices = [0..1] tflv;
        binding [1] bind [0..1] tsle.vertices = [0..1] trlv;

        binding [1] bind [0..1] bfe.vertices = [0..1] bflv;
        binding [1] bind [0..1] bfe.vertices = [0..1] bfrv;
        binding [1] bind [0..1] bre.vertices = [0..1] brlv;
        binding [1] bind [0..1] bre.vertices = [0..1] brrv;
        binding [1] bind [0..1] bsle.vertices = [0..1] bflv;
        binding [1] bind [0..1] bsle.vertices = [0..1] brlv;
        binding [1] bind [0..1] bsre.vertices = [0..1] bfrv;
        binding [1] bind [0..1] bsre.vertices = [0..1] brrv;

        binding [1] bind [0..1] ufle.vertices = [0..1] tflv;
        binding [1] bind [0..1] ufle.vertices = [0..1] bflv;
        binding [1] bind [0..1] urle.vertices = [0..1] trlv;
        binding [1] bind [0..1] urle.vertices = [0..1] brlv;

        /* Meeting edges */
        connection : MatesWith connect [1] tfe to [1] tfe;
        connection : MatesWith connect [1] tre to [1] tre;
        connection : MatesWith connect [1] tsle to [1] tsle;
        connection : MatesWith connect [1] bfe to [1] bfe;
        connection : MatesWith connect [1] bre to [1] bre;
        connection : MatesWith connect [1] bsle to [1] bsle;
        connection : MatesWith connect [1] bsre to [1] bsre;
        connection : MatesWith connect [1] ufle to [1] ufle;
        connection : MatesWith connect [1] urle to [1] urle;
        connection : MatesWith connect [1] bsre to [1] bsre;

        /* Meeting vertices  */
        connection : MatesWith connect [2] tflv to [2] tflv;
        connection : MatesWith connect [2] trlv to [2] trlv;
        connection : MatesWith connect [2] bflv to [2] bflv;
        connection : MatesWith connect [2] bfrv to [2] bfrv;
        connection : MatesWith connect [2] brlv to [2] brlv;
        connection : MatesWith connect [2] brrv to [2] brrv;
    }

    item def TriangularPrism :> CuboidOrTriangularPrism {
        doc /*
		 * A TriangularPrism is a Polyhedron with five sides, two triangular and
		 * the others quadrilateral.
		 */

        item :>> faces [5];
        item :>> ff : Triangle {
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;
        }
        item :>> rf : Triangle {
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;
        }

        item :>> edges [18];

        item :>> vertices;

        /* Bind face edges to specific edges */
        binding [1] bind [0..1] tf.edges = [0..1] bsre;

        /* Bind edge vertices to specific vertices */
        binding [1] bind [0..1] tfe.vertices = [0..1] bfrv;
        binding [1] bind [0..1] tre.vertices = [0..1] bfrv;
    }

    item def RightTriangularPrism :> TriangularPrism {
        doc /*
		 * A RightTriangularPrism  a TriangularPrism with two right triangluar sides,
		 * with given length, width, and height.
		 */

        attribute :>> length [1];
        attribute :>> width [1];
        attribute :>> height [1];

        item :>> tf : Rectangle;
        item :>> bf : Rectangle;
        item :>> ff : RightTriangle {
            attribute :>> length = RightTriangularPrism::length;
            attribute :>> width = RightTriangularPrism::width;
        }
        item :>> rf : RightTriangle {
            attribute :>> length = ff.length;
            attribute :>> width = rf.width;
        }
        item :>> slf : Rectangle;
        item :>> srf : Rectangle;

        item :>> tfe {
            attribute :>> length = ff.hypotenuse.length;
        }
        item :>> tre {
            attribute :>> length = tfe.length;
        }
        item :>> tsle {
            attribute :>> length = height;
        }
        item :>> bfe {
            attribute :>> length = RightTriangularPrism::length;
        }
        item :>> bre {
            attribute :>> length = RightTriangularPrism::length;
        }
        item :>> bsle {
            attribute :>> length = height;
        }
        item :>> bsre {
            attribute :>> length = height;
        }
        item :>> ufle {
            attribute :>> length = width;
        }
        item :>> urle {
            attribute :>> length = width;
        }
    }
    alias Wedge for RightTriangularPrism;

    item def Cuboid :> CuboidOrTriangularPrism {
        doc /*
		 * A Cuboid is a Polyhedron with six sides, all quadrilateral.
		 */

        item :>> faces [6];
        item :>> ff : Quadrilateral {
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;
        }
        item :>> rf : Quadrilateral {
            ref :>> Quadrilateral::edges, ConeOrCylinder::faces::edges;
            ref :>> Quadrilateral::vertices, ConeOrCylinder::faces::vertices;
        }

        item :>> edges [24];

        item :>> vertices;

        /* Bind face edges to specific edges */
        binding [1] bind [0..1] tf.edges = [0..1] tsre;
        binding [1] bind [0..1] ff.edges = [0..1] ufre;
        binding [1] bind [0..1] rf.edges = [0..1] urre;

        binding [1] bind [0..1] srf.edges = [0..1] tsre;
        binding [1] bind [0..1] srf.edges = [0..1] bsre;
        binding [1] bind [0..1] srf.edges = [0..1] ufre;
        binding [1] bind [0..1] srf.edges = [0..1] urre;

        /* Bind edge vertices to specific vertices */
        binding [1] bind [0..1] tfe.vertices = [0..1] tfrv;
        binding [1] bind [0..1] tre.vertices = [0..1] trrv;
        binding [1] bind [0..1] tsre.vertices = [0..1] tfrv;
        binding [1] bind [0..1] tsre.vertices = [0..1] trrv;

        binding [1] bind [0..1] ufre.vertices = [0..1] tfrv;
        binding [1] bind [0..1] ufre.vertices = [0..1] bfrv;
        binding [1] bind [0..1] urre.vertices = [0..1] trrv;
        binding [1] bind [0..1] urre.vertices = [0..1] brrv;

        /* Meeting edges */
        connection : MatesWith connect [1] tsre to [1] tsre;
        connection : MatesWith connect [1] ufre to [1] ufre;
        connection : MatesWith connect [1] urre to [1] urre;
        connection : MatesWith connect [1] bsre to [1] bsre;

        /* Meeting vertices  */
        connection : MatesWith connect [2] tfrv to [2] tfrv;
        connection : MatesWith connect [2] trrv to [2] trrv;
    }

    item def RectangularCuboid :> Cuboid {
        doc /*
		 * A RectangularCuboid is a Cuboid with all Rectangular sides.
		 */

        attribute :>> length [1];
        attribute :>> width [1];
        attribute :>> height [1];

        item :>> tf : Rectangle {
            attribute :>> length = RectangularCuboid::length;
            attribute :>> width = RectangularCuboid::height;
        }
        item :>> bf : Rectangle {
            attribute :>> length = RectangularCuboid::length;
            attribute :>> width = RectangularCuboid::height;
        }
        item :>> ff : Rectangle {
            attribute :>> length = RectangularCuboid::length;
            attribute :>> width = RectangularCuboid::width;
        }
        item :>> rf : Rectangle {
            attribute :>> length = RectangularCuboid::length;
            attribute :>> width = RectangularCuboid::width;
        }
        item :>> slf : Rectangle {
            attribute :>> length = RectangularCuboid::height;
            attribute :>> width = RectangularCuboid::width;
        }
        item :>> srf : Rectangle {
            attribute :>> length = RectangularCuboid::height;
            attribute :>> width = RectangularCuboid::width;
        }
    }
    alias Box for RectangularCuboid;

    item def Pyramid :> Polyhedron {
        doc /*
		 * A Pyramid is a Polyhedron with the sides of a polygon (base) forming the bases of triangles
		 * that join at an apex point.	Its height is the perpendicular distance from the base to the apex,
		 * and its offsets are between this perpendicular at the base and the center of the base.
		 */

        attribute :>> height [1];
        attribute :>> xoffset;
        attribute :>> yoffset;

        item :>> faces;
        item base :> faces [1];
        item wall : Triangle :> faces {
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;
        }
        attribute wallNumber : Positive = size(wall);

        assert constraint {
            = size(faces) == wallNumber + 1;
        }
        assert constraint {
            = size(wall) == size(base.edges);
        }

        item :>> edges;

        assert constraint {
            = size(edges) == wallNumber * 4;
        }

        item :>> vertices;
        item apex :> vertices = wall.apex;

        assert constraint {
            = size(apex) == wallNumber;
        }

        /* Base to wall and wall to wall edge mating. */
        assert constraint {
            = (1 .. wallNumber)->forAll {in i;
					includes(wall#(i).base.matingOccurrences,
							 Pyramid::base.edges#(i)) and
					includes((wall#(i).edges#(3) as Item).matingOccurrences,
							 wall#(if i==wallNumber ? 1 else i+1).edges#(2)) };
        }

        /* Meeting apices. */
        connection : MatesWith connect [wallNumber] apex to [wallNumber] apex;
    }

    item def Tetrahedron :> Pyramid {
        doc /*
		 * A Tetrahedron is Pyramid with a triangular base.
		 */

        attribute :>> baseLength [1];
        attribute :>> baseWidth [1];

        item :>> base : Triangle {
            ref :>> Triangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Triangle::vertices, ConeOrCylinder::faces::vertices;
            attribute :>> length = Tetrahedron::baseLength;
            attribute :>> width = Tetrahedron::baseWidth;
        }
    }

    item def RectangularPyramid :> Pyramid {
        doc /*
		 * A RectangularPyramid is Pyramid with a rectangular base.
		 */

        attribute :>> baseLength [1];
        attribute :>> baseWidth [1];

        item :>> base : Rectangle {
            ref :>> Rectangle::edges, ConeOrCylinder::faces::edges;
            ref :>> Rectangle::vertices, ConeOrCylinder::faces::vertices;
            attribute :>> length = RectangularPyramid::baseLength;
            attribute :>> width = RectangularPyramid::baseWidth;
        }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'ShapeItems'
      (documentation)
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarValues::Positive'[unresolved])
      (namespace_import private -> 'ISQSpaceTime'[unresolved])
      (namespace_import private -> 'ISQBase'[unresolved])
      (membership_import private -> 'SI::m'[unresolved])
      (membership_import private -> 'Occurrences::MatesWith'[unresolved])
      (namespace_import private -> 'Objects'[unresolved])
      (membership_import private -> 'Items::Item'[unresolved])
      (membership_import private -> 'SequenceFunctions::equals'[unresolved])
      (membership_import private -> 'SequenceFunctions::isEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::notEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (membership_import private -> 'SequenceFunctions::includes'[unresolved])
      (membership_import private -> 'ControlFunctions::if'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (membership_import private -> 'ControlFunctions::exists'[unresolved])
      (membership_import private -> 'Quantities::scalarQuantities'[unresolved])
      (item_def 'PlanarCurve' :> 'Curve'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'length'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'outerSpaceDimension'[unresolved])
        (assert_constraint_usage
          (result_expr_membership)))
      (item_def 'PlanarSurface' :> 'Surface'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'area'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'outerSpaceDimension'[unresolved]
          (feature_value (=)))
        (item_usage composite :>> 'shape'[unresolved] : 'ShapeItems::PlanarCurve'[item_def]))
      (item_def 'Line' :> 'ShapeItems::PlanarCurve'[item_def]
        (documentation)
        (attribute_usage composite :>> 'length'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'outerSpaceDimension'[unresolved]
          (feature_value (=))))
      (item_def abstract 'Path' :> 'StructuredSpaceObject::StructuredCurve'[unresolved]
        (documentation)
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [0]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [1..*])
          (item_usage composite :>> 'vertices'[unresolved]
            (multiplicity_range [0..2])))
        (item_usage composite :>> 'vertices'[unresolved]
          (multiplicity_range [*])
          (feature_value (=)))
        (assert_constraint_usage
          (result_expr_membership)))
      (attribute_usage 'semiMajorAxis' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [0..*]))
      (attribute_usage 'semiMinorAxis' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [0..*]))
      (attribute_usage 'xoffset' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [0..*])
        (feature_value (default =)))
      (attribute_usage 'yoffset' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [0..*])
        (feature_value (default =)))
      (attribute_usage 'baseLength' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [0..*]))
      (attribute_usage 'baseWidth' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [0..*]))
      (item_def 'ConicSection' :> 'ShapeItems::Path'[item_def] :> 'ShapeItems::PlanarCurve'[item_def]
        (documentation)
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [1..2]))
        (item_usage composite :>> 'vertices'[unresolved]
          (multiplicity_range [0])))
      (item_def 'Ellipse' :> 'ShapeItems::ConicSection'[item_def]
        (documentation)
        (attribute_usage composite :>> 'ShapeItems::semiMajorAxis'[attribute_usage]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::semiMinorAxis'[attribute_usage]
          (multiplicity_range [1]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [1])))
      (item_def 'Circle' :> 'ShapeItems::Ellipse'[item_def]
        (documentation)
        (attribute_usage composite :>> 'radius'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> ''[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite :>> ''[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (item_usage composite :>> 'edges'[unresolved]
          (attribute_usage composite 'length'
            (multiplicity_range [1])
            (feature_value (=)))))
      (item_def 'Parabola' :> 'ShapeItems::ConicSection'[item_def]
        (documentation)
        (attribute_usage composite 'focalDistance' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [1])))
      (item_def 'Hyperbola' :> 'ShapeItems::ConicSection'[item_def]
        (documentation)
        (attribute_usage composite 'tranverseAxis' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'conjugateAxis' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1])))
      (item_def 'Polygon' :> 'ShapeItems::Path'[item_def] :> 'ShapeItems::PlanarCurve'[item_def]
        (documentation)
        (item_usage composite :>> 'edges'[unresolved] : 'ShapeItems::Line'[item_def]
          (item_usage composite :>> 'vertices'[unresolved]
            (multiplicity_range [2])))
        (attribute_usage composite :>> 'isClosed'[unresolved]
          (feature_value (=)))
        (assert_constraint_usage
          (result_expr_membership)))
      (item_def 'Triangle' :> 'ShapeItems::Polygon'[item_def]
        (documentation)
        (attribute_usage composite :>> 'length'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'width'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::xoffset'[attribute_usage]
          (multiplicity_range [1]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [3])
          (feature_value (=)))
        (item_usage composite 'base'
          (multiplicity_range [1])
          (reference_usage reference 'length'
            (feature_value (=))))
        (item_usage composite 'e2'
          (multiplicity_range [1]))
        (item_usage composite 'e3'
          (multiplicity_range [1]))
        (item_usage composite :>> 'vertices'[unresolved]
          (multiplicity_range [6]))
        (item_usage composite ordered 'v12'
          (multiplicity_range [2])
          (feature_value (=)))
        (item_usage composite ordered 'apex'
          (multiplicity_range [2])
          (feature_value (=)))
        (item_usage composite ordered 'v31'
          (multiplicity_range [2])
          (feature_value (=))))
      (item_def 'RightTriangle' :> 'ShapeItems::Triangle'[item_def]
        (documentation)
        (attribute_usage composite :>> ''[attribute_usage]
          (feature_value (=)))
        (item_usage composite :>> 'ShapeItems::Triangle::e2'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite 'hypotenuse' :>> 'ShapeItems::Triangle::e3'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))))
      (item_def 'Quadrilateral' :> 'ShapeItems::Polygon'[item_def]
        (documentation)
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [4])
          (feature_value (=)))
        (item_usage composite 'e1'
          (multiplicity_range [1]))
        (item_usage composite 'e2'
          (multiplicity_range [1]))
        (item_usage composite 'e3'
          (multiplicity_range [1]))
        (item_usage composite 'e4'
          (multiplicity_range [1]))
        (item_usage composite :>> 'vertices'[unresolved]
          (multiplicity_range [8]))
        (item_usage composite ordered 'v12'
          (multiplicity_range [2])
          (feature_value (=)))
        (item_usage composite ordered 'v23'
          (multiplicity_range [2])
          (feature_value (=)))
        (item_usage composite ordered 'v34'
          (multiplicity_range [2])
          (feature_value (=)))
        (item_usage composite ordered 'v41'
          (multiplicity_range [2])
          (feature_value (=))))
      (item_def 'Rectangle' :> 'ShapeItems::Quadrilateral'[item_def]
        (documentation)
        (attribute_usage composite :>> 'length'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'width'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'ShapeItems::Quadrilateral::e1'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::Quadrilateral::e2'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::Quadrilateral::e3'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::Quadrilateral::e4'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))))
      (item_def abstract 'Shell' :> 'StructuredSpaceObject::StructuredSurface'[unresolved]
        (documentation))
      (item_def 'Disc' :> 'ShapeItems::Shell'[item_def] :> 'ShapeItems::PlanarSurface'[item_def]
        (documentation)
        (attribute_usage composite :>> 'ShapeItems::semiMajorAxis'[attribute_usage]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::semiMinorAxis'[attribute_usage]
          (multiplicity_range [1]))
        (item_usage composite :>> 'shape'[unresolved] : 'ShapeItems::Ellipse'[item_def]
          (multiplicity_range [1])
          (attribute_usage composite :>> ''[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> ''[attribute_usage]
            (feature_value (=))))
        (item_usage composite :>> 'faces'[unresolved] : 'ShapeItems::PlanarSurface'[item_def]
          (multiplicity_range [1])
          (item_usage composite :>> 'edges'[unresolved]
            (multiplicity_range [1])))
        (item_usage composite :>> 'edges'[unresolved] : 'ShapeItems::Ellipse'[item_def]
          (multiplicity_range [1])
          (feature_value (=))
          (attribute_usage composite :>> 'Shell::edges::innerSpaceDimension'[unresolved] :>> 'Ellipse::innerSpaceDimension'[unresolved])
          (item_usage reference :>> 'Shell::edges::vertices'[unresolved] :>> 'Ellipse::vertices'[unresolved]))
        (item_usage composite :>> 'vertices'[unresolved]
          (multiplicity_range [0])))
      (item_def 'CircularDisc' :> 'ShapeItems::Disc'[item_def]
        (documentation)
        (attribute_usage composite :>> 'radius'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> ''[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite :>> ''[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (item_usage composite :>> 'shape'[unresolved] : 'ShapeItems::Circle'[item_def]
          (attribute_usage composite :>> 'Disc::shape::semiMajorAxis'[unresolved] :>> ''[attribute_usage])
          (attribute_usage composite :>> 'Disc::shape::semiMinorAxis'[unresolved] :>> ''[attribute_usage]))
        (item_usage composite :>> 'edges'[unresolved] : 'ShapeItems::Circle'[item_def]))
      (item_def 'ConicSurface' :> 'ShapeItems::Shell'[item_def]
        (documentation)
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [1..2]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [0]))
        (item_usage composite :>> 'vertices'[unresolved]
          (multiplicity_range [0]))
        (attribute_usage composite :>> 'genus'[unresolved]
          (feature_value (=))))
      (item_def 'Ellipsoid' :> 'ShapeItems::ConicSurface'[item_def]
        (documentation)
        (attribute_usage composite 'semiAxis1' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'semiAxis2' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'semiAxis3' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [1])))
      (item_def 'Sphere' :> 'ShapeItems::Ellipsoid'[item_def]
        (documentation)
        (attribute_usage composite :>> 'radius'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::Ellipsoid::semiAxis1'[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite :>> 'ShapeItems::Ellipsoid::semiAxis2'[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite :>> 'ShapeItems::Ellipsoid::semiAxis3'[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=))))
      (item_def 'Paraboloid' :> 'ShapeItems::ConicSurface'[item_def]
        (documentation)
        (attribute_usage composite 'focalDistance' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [1])))
      (item_def 'Hyperboloid' :> 'ShapeItems::ConicSurface'[item_def]
        (documentation)
        (attribute_usage composite 'transverseAxis' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'conjugateAxis' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1])))
      (item_def 'Toroid' :> 'ShapeItems::Shell'[item_def]
        (documentation)
        (attribute_usage composite 'revolutionRadius' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite 'revolvedCurve' : 'ShapeItems::PlanarCurve'[item_def]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'isClosed'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [0]))
        (item_usage composite :>> 'vertices'[unresolved]
          (multiplicity_range [0]))
        (attribute_usage composite :>> 'genus'[unresolved]
          (feature_value (=))))
      (item_def 'Torus' :> 'ShapeItems::Toroid'[item_def]
        (documentation)
        (attribute_usage composite 'majorRadius' :>> 'ShapeItems::Toroid::revolutionRadius'[attribute_usage])
        (attribute_usage composite 'minorRadius' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'ShapeItems::Toroid::revolvedCurve'[item_usage] : 'ShapeItems::Circle'[item_def]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'radius'[unresolved]
            (feature_value (=)))))
      (item_def 'RectangularToroid' :> 'ShapeItems::Toroid'[item_def]
        (documentation)
        (attribute_usage composite 'rectangleLength' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'rectangleWidth' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'ShapeItems::Toroid::revolvedCurve'[item_usage] : 'ShapeItems::Rectangle'[item_def]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'revolvedCurve::isClosed'[unresolved] :>> 'Rectangle::isClosed'[unresolved])))
      (item_def 'ConeOrCylinder' :> 'ShapeItems::Shell'[item_def]
        (documentation)
        (attribute_usage composite :>> 'ShapeItems::semiMajorAxis'[attribute_usage]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::semiMinorAxis'[attribute_usage]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'height'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::xoffset'[attribute_usage]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::yoffset'[attribute_usage]
          (multiplicity_range [1]))
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [2..3]))
        (item_usage composite 'base' : 'ShapeItems::Disc'[item_def] :> 'faces'[unresolved]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'Disc::innerSpaceDimension'[unresolved] :>> 'faces::innerSpaceDimension'[unresolved])
          (reference_usage reference :>> 'Disc::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved]
            (attribute_usage composite :>> 'Disc::edges::innerSpaceDimension'[unresolved] :>> 'ConeOrCylinder::faces::edges::innerSpaceDimension'[unresolved]))
          (reference_usage reference :>> 'Disc::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite 'af' : 'ShapeItems::Disc'[item_def] :> 'faces'[unresolved]
          (multiplicity_range [0..1])
          (attribute_usage composite :>> 'Disc::innerSpaceDimension'[unresolved] :>> 'faces::innerSpaceDimension'[unresolved])
          (reference_usage reference :>> 'Disc::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved]
            (attribute_usage composite :>> 'Disc::edges::innerSpaceDimension'[unresolved] :>> 'ConeOrCylinder::faces::edges::innerSpaceDimension'[unresolved]))
          (reference_usage reference :>> 'Disc::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite 'cf' : 'Surface'[unresolved] :> 'faces'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [2..4])
          (feature_value (=)))
        (item_usage composite 'be' :> 'edges'[unresolved]
          (multiplicity_range [2])
          (attribute_usage composite :>> ''[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> ''[attribute_usage]
            (feature_value (=))))
        (item_usage composite 'ae' :> 'edges'[unresolved]
          (multiplicity_range [0..2])
          (attribute_usage composite :>> ''[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> ''[attribute_usage]
            (feature_value (=))))
        (assert_constraint_usage
          (result_expr_membership))
        (item_usage composite :>> 'vertices'[unresolved]
          (multiplicity_range [0..1])
          (feature_value (=)))
        (assert_constraint_usage
          (result_expr_membership))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'base.edges')
          (connector_end 'be'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'cf.edges')
          (connector_end 'be'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'be')
          (connector_end 'be'))
        (attribute_usage composite :>> 'genus'[unresolved]
          (feature_value (=))))
      (item_def 'Cone' :> 'ShapeItems::ConeOrCylinder'[item_def]
        (documentation)
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'apex' :>> 'vertices'[unresolved])
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'cf.vertices')
          (connector_end 'apex')))
      (item_def 'EccentricCone' :> 'ShapeItems::Cone'[item_def]
        (documentation)
        (assert_constraint_usage
          (result_expr_membership)))
      (item_def 'CircularCone' :> 'ShapeItems::Cone'[item_def]
        (documentation)
        (attribute_usage composite :>> 'radius'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> ''[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite :>> ''[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (item_usage composite :>> 'ShapeItems::ConeOrCylinder::base'[item_usage] : 'ShapeItems::CircularDisc'[item_def]
          (reference_usage reference :>> 'base::edges'[unresolved] :>> 'CircularDisc::edges'[unresolved])))
      (item_def 'RightCircularCone' :> 'ShapeItems::CircularCone'[item_def]
        (documentation)
        (attribute_usage composite :>> ''[attribute_usage]
          (attribute_usage composite :>> 'num'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> ''[attribute_usage]
          (attribute_usage composite :>> 'num'[unresolved]
            (feature_value (=)))))
      (item_def 'Cylinder' :> 'ShapeItems::ConeOrCylinder'[item_def]
        (documentation)
        (item_usage composite :>> 'ShapeItems::ConeOrCylinder::af'[item_usage]
          (multiplicity_range [1]))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'cf.edges')
          (connector_end 'ae'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'ae')
          (connector_end 'ae')
          (documentation)))
      (item_def 'EccentricCylinder' :> 'ShapeItems::Cylinder'[item_def]
        (documentation)
        (assert_constraint_usage
          (result_expr_membership)))
      (item_def 'CircularCylinder' :> 'ShapeItems::Cylinder'[item_def]
        (documentation)
        (attribute_usage composite :>> 'radius'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> ''[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite :>> ''[attribute_usage]
          (multiplicity_range [1])
          (feature_value (=)))
        (item_usage composite :>> 'ShapeItems::ConeOrCylinder::base'[item_usage] : 'ShapeItems::CircularDisc'[item_def]
          (reference_usage reference :>> 'base::edges'[unresolved] :>> 'CircularDisc::edges'[unresolved]))
        (item_usage composite :>> ''[item_usage] : 'ShapeItems::CircularDisc'[item_def]
          (reference_usage reference :>> 'af::edges'[unresolved] :>> 'CircularDisc::edges'[unresolved])))
      (item_def 'RightCircularCylinder' :> 'ShapeItems::CircularCylinder'[item_def]
        (documentation)
        (attribute_usage composite :>> ''[attribute_usage]
          (attribute_usage composite :>> 'num'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> ''[attribute_usage]
          (attribute_usage composite :>> 'num'[unresolved]
            (feature_value (=)))))
      (item_def 'Polyhedron' :> 'ShapeItems::Shell'[item_def]
        (documentation)
        (attribute_usage composite :>> 'isClosed'[unresolved]
          (feature_value (=)))
        (item_usage composite :>> 'faces'[unresolved] : 'ShapeItems::Polygon'[item_def]
          (multiplicity_range [2..*])
          (attribute_usage composite :>> 'Polygon::innerSpaceDimension'[unresolved] :>> 'faces::innerSpaceDimension'[unresolved])
          (reference_usage reference :>> 'Polygon::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Polygon::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite :>> 'edges'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'outerSpaceDimension'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'genus'[unresolved]
          (feature_value (=))))
      (item_def 'CuboidOrTriangularPrism' :> 'ShapeItems::Polyhedron'[item_def]
        (documentation)
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [5..6]))
        (item_usage composite 'tf' : 'ShapeItems::Quadrilateral'[item_def] :> 'faces'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'Quadrilateral::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Quadrilateral::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite 'bf' : 'ShapeItems::Quadrilateral'[item_def] :> 'faces'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'Quadrilateral::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Quadrilateral::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite 'ff' : 'ShapeItems::Polygon'[item_def] :> 'faces'[unresolved]
          (multiplicity_range [1])
          (item_usage composite :>> 'Polygon::edges'[unresolved] :>> 'faces::edges'[unresolved]
            (multiplicity_range [3..4])))
        (item_usage composite 'rf' : 'ShapeItems::Polygon'[item_def] :> 'faces'[unresolved]
          (multiplicity_range [1])
          (item_usage composite :>> 'Polygon::edges'[unresolved] :>> 'faces::edges'[unresolved]
            (multiplicity_range [3..4])))
        (item_usage composite 'slf' : 'ShapeItems::Quadrilateral'[item_def] :> 'faces'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'Quadrilateral::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Quadrilateral::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite 'srf' : 'ShapeItems::Quadrilateral'[item_def] :> 'faces'[unresolved]
          (multiplicity_range [0..1])
          (reference_usage reference :>> 'Quadrilateral::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Quadrilateral::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite :>> 'edges'[unresolved])
        (assert_constraint_usage
          (result_expr_membership))
        (item_usage composite 'tfe' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'tre' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'tsle' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'tsre' :> 'edges'[unresolved]
          (multiplicity_range [0..2]))
        (item_usage composite 'bfe' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'bre' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'bsle' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'bsre' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'ufle' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'ufre' :> 'edges'[unresolved]
          (multiplicity_range [0..2]))
        (item_usage composite 'urle' :> 'edges'[unresolved]
          (multiplicity_range [2]))
        (item_usage composite 'urre' :> 'edges'[unresolved]
          (multiplicity_range [0..2]))
        (assert_constraint_usage
          (result_expr_membership))
        (item_usage composite :>> 'vertices'[unresolved])
        (assert_constraint_usage
          (result_expr_membership))
        (item_usage composite 'tflv' :> 'vertices'[unresolved]
          (multiplicity_range [3]))
        (item_usage composite 'tfrv' :> 'vertices'[unresolved]
          (multiplicity_range [0..3]))
        (item_usage composite 'trlv' :> 'vertices'[unresolved]
          (multiplicity_range [3]))
        (item_usage composite 'trrv' :> 'vertices'[unresolved]
          (multiplicity_range [0..3]))
        (item_usage composite 'bflv' :> 'vertices'[unresolved]
          (multiplicity_range [3]))
        (item_usage composite 'bfrv' :> 'vertices'[unresolved]
          (multiplicity_range [3]))
        (item_usage composite 'brlv' :> 'vertices'[unresolved]
          (multiplicity_range [3]))
        (item_usage composite 'brrv' :> 'vertices'[unresolved]
          (multiplicity_range [3]))
        (assert_constraint_usage
          (result_expr_membership))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tf.edges')
          (connector_end 'tfe'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tf.edges')
          (connector_end 'tre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tf.edges')
          (connector_end 'tsle'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bf.edges')
          (connector_end 'bfe'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bf.edges')
          (connector_end 'bre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bf.edges')
          (connector_end 'bsle'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bf.edges')
          (connector_end 'bsre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'ff.edges')
          (connector_end 'tfe'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'ff.edges')
          (connector_end 'bfe'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'ff.edges')
          (connector_end 'ufle'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'rf.edges')
          (connector_end 'tre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'rf.edges')
          (connector_end 'bre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'rf.edges')
          (connector_end 'urle'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tfe.vertices')
          (connector_end 'tflv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tre.vertices')
          (connector_end 'trlv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tsle.vertices')
          (connector_end 'tflv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tsle.vertices')
          (connector_end 'trlv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bfe.vertices')
          (connector_end 'bflv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bfe.vertices')
          (connector_end 'bfrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bre.vertices')
          (connector_end 'brlv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bre.vertices')
          (connector_end 'brrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bsle.vertices')
          (connector_end 'bflv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bsle.vertices')
          (connector_end 'brlv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bsre.vertices')
          (connector_end 'bfrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'bsre.vertices')
          (connector_end 'brrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'ufle.vertices')
          (connector_end 'tflv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'ufle.vertices')
          (connector_end 'bflv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'urle.vertices')
          (connector_end 'trlv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'urle.vertices')
          (connector_end 'brlv'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'tfe')
          (connector_end 'tfe'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'tre')
          (connector_end 'tre'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'tsle')
          (connector_end 'tsle'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'bfe')
          (connector_end 'bfe'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'bre')
          (connector_end 'bre'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'bsle')
          (connector_end 'bsle'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'bsre')
          (connector_end 'bsre'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'ufle')
          (connector_end 'ufle'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'urle')
          (connector_end 'urle'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'bsre')
          (connector_end 'bsre'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'tflv')
          (connector_end 'tflv'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'trlv')
          (connector_end 'trlv'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'bflv')
          (connector_end 'bflv'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'bfrv')
          (connector_end 'bfrv'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'brlv')
          (connector_end 'brlv'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'brrv')
          (connector_end 'brrv')))
      (item_def 'TriangularPrism' :> 'ShapeItems::CuboidOrTriangularPrism'[item_def]
        (documentation)
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [5]))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::ff'[item_usage] : 'ShapeItems::Triangle'[item_def]
          (reference_usage reference :>> 'Triangle::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Triangle::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::rf'[item_usage] : 'ShapeItems::Triangle'[item_def]
          (reference_usage reference :>> 'Triangle::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Triangle::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [18]))
        (item_usage composite :>> 'vertices'[unresolved])
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tf.edges')
          (connector_end 'bsre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tfe.vertices')
          (connector_end 'bfrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tre.vertices')
          (connector_end 'bfrv')))
      (item_def 'RightTriangularPrism' :> 'ShapeItems::TriangularPrism'[item_def]
        (documentation)
        (attribute_usage composite :>> 'length'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'width'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'height'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::tf'[item_usage] : 'ShapeItems::Rectangle'[item_def])
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::bf'[item_usage] : 'ShapeItems::Rectangle'[item_def])
        (item_usage composite :>> ''[item_usage] : 'ShapeItems::RightTriangle'[item_def]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> ''[item_usage] : 'ShapeItems::RightTriangle'[item_def]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::slf'[item_usage] : 'ShapeItems::Rectangle'[item_def])
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::srf'[item_usage] : 'ShapeItems::Rectangle'[item_def])
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::tfe'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::tre'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::tsle'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::bfe'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::bre'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::bsle'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::bsre'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::ufle'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::urle'[item_usage]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))))
      (alias_member 'Wedge' -> 'ShapeItems::RightTriangularPrism'[item_def])
      (item_def 'Cuboid' :> 'ShapeItems::CuboidOrTriangularPrism'[item_def]
        (documentation)
        (item_usage composite :>> 'faces'[unresolved]
          (multiplicity_range [6]))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::ff'[item_usage] : 'ShapeItems::Quadrilateral'[item_def]
          (reference_usage reference :>> 'Quadrilateral::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Quadrilateral::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::rf'[item_usage] : 'ShapeItems::Quadrilateral'[item_def]
          (reference_usage reference :>> 'Quadrilateral::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Quadrilateral::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (item_usage composite :>> 'edges'[unresolved]
          (multiplicity_range [24]))
        (item_usage composite :>> 'vertices'[unresolved])
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tf.edges')
          (connector_end 'tsre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'ff.edges')
          (connector_end 'ufre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'rf.edges')
          (connector_end 'urre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'srf.edges')
          (connector_end 'tsre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'srf.edges')
          (connector_end 'bsre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'srf.edges')
          (connector_end 'ufre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'srf.edges')
          (connector_end 'urre'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tfe.vertices')
          (connector_end 'tfrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tre.vertices')
          (connector_end 'trrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tsre.vertices')
          (connector_end 'tfrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'tsre.vertices')
          (connector_end 'trrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'ufre.vertices')
          (connector_end 'tfrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'ufre.vertices')
          (connector_end 'bfrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'urre.vertices')
          (connector_end 'trrv'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'urre.vertices')
          (connector_end 'brrv'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'tsre')
          (connector_end 'tsre'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'ufre')
          (connector_end 'ufre'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'urre')
          (connector_end 'urre'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'bsre')
          (connector_end 'bsre'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'tfrv')
          (connector_end 'tfrv'))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'trrv')
          (connector_end 'trrv')))
      (item_def 'RectangularCuboid' :> 'ShapeItems::Cuboid'[item_def]
        (documentation)
        (attribute_usage composite :>> 'length'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'width'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'height'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::tf'[item_usage] : 'ShapeItems::Rectangle'[item_def]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::bf'[item_usage] : 'ShapeItems::Rectangle'[item_def]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> ''[item_usage] : 'ShapeItems::Rectangle'[item_def]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> ''[item_usage] : 'ShapeItems::Rectangle'[item_def]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::slf'[item_usage] : 'ShapeItems::Rectangle'[item_def]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=))))
        (item_usage composite :>> 'ShapeItems::CuboidOrTriangularPrism::srf'[item_usage] : 'ShapeItems::Rectangle'[item_def]
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=)))))
      (alias_member 'Box' -> 'ShapeItems::RectangularCuboid'[item_def])
      (item_def 'Pyramid' :> 'ShapeItems::Polyhedron'[item_def]
        (documentation)
        (attribute_usage composite :>> 'height'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::xoffset'[attribute_usage])
        (attribute_usage composite :>> 'ShapeItems::yoffset'[attribute_usage])
        (item_usage composite :>> 'faces'[unresolved])
        (item_usage composite 'base' :> 'faces'[unresolved]
          (multiplicity_range [1]))
        (item_usage composite 'wall' : 'ShapeItems::Triangle'[item_def] :> 'faces'[unresolved]
          (reference_usage reference :>> 'Triangle::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Triangle::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved]))
        (attribute_usage composite 'wallNumber' : 'Positive'[unresolved]
          (feature_value (=)))
        (assert_constraint_usage
          (result_expr_membership))
        (assert_constraint_usage
          (result_expr_membership))
        (item_usage composite :>> 'edges'[unresolved])
        (assert_constraint_usage
          (result_expr_membership))
        (item_usage composite :>> 'vertices'[unresolved])
        (item_usage composite 'apex' :> 'vertices'[unresolved]
          (feature_value (=)))
        (assert_constraint_usage
          (result_expr_membership))
        (assert_constraint_usage
          (result_expr_membership))
        (connection_usage composite : 'MatesWith'[unresolved]
          (connector_end 'apex')
          (connector_end 'apex')))
      (item_def 'Tetrahedron' :> 'ShapeItems::Pyramid'[item_def]
        (documentation)
        (attribute_usage composite :>> 'ShapeItems::baseLength'[attribute_usage]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::baseWidth'[attribute_usage]
          (multiplicity_range [1]))
        (item_usage composite :>> 'ShapeItems::Pyramid::base'[item_usage] : 'ShapeItems::Triangle'[item_def]
          (reference_usage reference :>> 'Triangle::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Triangle::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved])
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=)))))
      (item_def 'RectangularPyramid' :> 'ShapeItems::Pyramid'[item_def]
        (documentation)
        (attribute_usage composite :>> 'ShapeItems::baseLength'[attribute_usage]
          (multiplicity_range [1]))
        (attribute_usage composite :>> 'ShapeItems::baseWidth'[attribute_usage]
          (multiplicity_range [1]))
        (item_usage composite :>> 'ShapeItems::Pyramid::base'[item_usage] : 'ShapeItems::Rectangle'[item_def]
          (reference_usage reference :>> 'Rectangle::edges'[unresolved] :>> 'ConeOrCylinder::faces::edges'[unresolved])
          (reference_usage reference :>> 'Rectangle::vertices'[unresolved] :>> 'ConeOrCylinder::faces::vertices'[unresolved])
          (attribute_usage composite :>> 'length'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'width'[unresolved]
            (feature_value (=))))))))
~~~
