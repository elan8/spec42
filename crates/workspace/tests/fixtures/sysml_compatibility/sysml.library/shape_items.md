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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ShapeItems"))) (name "ShapeItems") (declared-name "ShapeItems")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ShapeItems::Box"))) (name "Box") (declared-name "Box"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Circle"))) (name "Circle") (declared-name "Circle")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Circle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Circle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Circle::radius"))) (name "radius") (declared-name "radius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Circle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (name "semiMajorAxis") (declared-name "semiMajorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Circle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (name "semiMinorAxis") (declared-name "semiMinorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Circle")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (name "CircularCone") (declared-name "CircularCone")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularCone")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))) (name "radius") (declared-name "radius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularCone")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (name "semiMajorAxis") (declared-name "semiMajorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularCone")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (name "semiMinorAxis") (declared-name "semiMinorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularCone")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (name "CircularCylinder") (declared-name "CircularCylinder")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))) (name "radius") (declared-name "radius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (name "semiMajorAxis") (declared-name "semiMajorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (name "semiMinorAxis") (declared-name "semiMinorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularCylinder")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (name "CircularDisc") (declared-name "CircularDisc")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularDisc")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))) (name "radius") (declared-name "radius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularDisc")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (name "semiMajorAxis") (declared-name "semiMajorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularDisc")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (name "semiMinorAxis") (declared-name "semiMinorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::CircularDisc")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Cone"))) (name "Cone") (declared-name "Cone")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Cone::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Cone")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (name "ConeOrCylinder") (declared-name "ConeOrCylinder")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))) (name "genus") (declared-name "genus") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))) (name "semiMajorAxis") (declared-name "semiMajorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))) (name "semiMinorAxis") (declared-name "semiMinorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))) (name "xoffset") (declared-name "xoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))) (name "yoffset") (declared-name "yoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (name "ConicSection") (declared-name "ConicSection")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::ConicSection::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConicSection")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (name "ConicSurface") (declared-name "ConicSurface")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConicSurface")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))) (name "genus") (declared-name "genus") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::ConicSurface")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (name "Cuboid") (declared-name "Cuboid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Cuboid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Cuboid")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (name "CuboidOrTriangularPrism") (declared-name "CuboidOrTriangularPrism")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (name "Cylinder") (declared-name "Cylinder")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Cylinder::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Cylinder")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Disc"))) (name "Disc") (declared-name "Disc")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Disc::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Disc")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))) (name "semiMajorAxis") (declared-name "semiMajorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Disc")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))) (name "semiMinorAxis") (declared-name "semiMinorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Disc")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))) (name "EccentricCone") (declared-name "EccentricCone")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::EccentricCone::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::EccentricCone")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))) (name "EccentricCylinder") (declared-name "EccentricCylinder")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (name "Ellipse") (declared-name "Ellipse")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Ellipse")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))) (name "semiMajorAxis") (declared-name "semiMajorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Ellipse")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))) (name "semiMinorAxis") (declared-name "semiMinorAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Ellipse")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (name "Ellipsoid") (declared-name "Ellipsoid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Ellipsoid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (name "semiAxis1") (declared-name "semiAxis1") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Ellipsoid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (name "semiAxis2") (declared-name "semiAxis2") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Ellipsoid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (name "semiAxis3") (declared-name "semiAxis3") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Ellipsoid")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (name "Hyperbola") (declared-name "Hyperbola")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Hyperbola")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (name "conjugateAxis") (declared-name "conjugateAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Hyperbola")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (name "tranverseAxis") (declared-name "tranverseAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Hyperbola")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (name "Hyperboloid") (declared-name "Hyperboloid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Hyperboloid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (name "conjugateAxis") (declared-name "conjugateAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Hyperboloid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (name "transverseAxis") (declared-name "transverseAxis") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Hyperboloid")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::Item"))) (name "Item") (declared-name "Item"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Line"))) (name "Line") (declared-name "Line")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Line::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Line")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (name "length") (declared-name "length") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Line")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Line")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::MatesWith"))) (name "MatesWith") (declared-name "MatesWith"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (name "Parabola") (declared-name "Parabola")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Parabola::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Parabola")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (name "focalDistance") (declared-name "focalDistance") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Parabola")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (name "Paraboloid") (declared-name "Paraboloid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Paraboloid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (name "focalDistance") (declared-name "focalDistance") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Paraboloid")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Path"))) (name "Path") (declared-name "Path")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Path::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Path")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (name "PlanarCurve") (declared-name "PlanarCurve")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))) (name "length") (declared-name "length") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))) (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (name "PlanarSurface") (declared-name "PlanarSurface")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::PlanarSurface")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))) (name "area") (declared-name "area") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::PlanarSurface")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))) (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::PlanarSurface")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (name "Polygon") (declared-name "Polygon")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Polygon::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Polygon")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))) (name "isClosed") (declared-name "isClosed") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Polygon")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (name "Polyhedron") (declared-name "Polyhedron")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Polyhedron")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))) (name "genus") (declared-name "genus") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Polyhedron")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))) (name "isClosed") (declared-name "isClosed") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Polyhedron")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))) (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Polyhedron")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::Positive"))) (name "Positive") (declared-name "Positive"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (name "Pyramid") (declared-name "Pyramid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (name "wallNumber") (declared-name "wallNumber") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))) (name "xoffset") (declared-name "xoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))) (name "yoffset") (declared-name "yoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (name "Quadrilateral") (declared-name "Quadrilateral")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Quadrilateral::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Quadrilateral")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (name "Rectangle") (declared-name "Rectangle")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Rectangle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (name "length") (declared-name "length") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Rectangle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::width"))) (name "width") (declared-name "width") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Rectangle")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (name "RectangularCuboid") (declared-name "RectangularCuboid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))) (name "length") (declared-name "length") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))) (name "width") (declared-name "width") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (name "RectangularPyramid") (declared-name "RectangularPyramid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))) (name "baseLength") (declared-name "baseLength") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))) (name "baseWidth") (declared-name "baseWidth") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (name "RectangularToroid") (declared-name "RectangularToroid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularToroid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (name "rectangleLength") (declared-name "rectangleLength") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularToroid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (name "rectangleWidth") (declared-name "rectangleWidth") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RectangularToroid")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (name "RightCircularCone") (declared-name "RightCircularCone")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightCircularCone")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (name "xoffset") (declared-name "xoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightCircularCone")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (name "yoffset") (declared-name "yoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightCircularCone")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (name "RightCircularCylinder") (declared-name "RightCircularCylinder")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (name "xoffset") (declared-name "xoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (name "yoffset") (declared-name "yoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (name "RightTriangle") (declared-name "RightTriangle")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightTriangle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (name "xoffset") (declared-name "xoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightTriangle")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (name "RightTriangularPrism") (declared-name "RightTriangularPrism")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))) (name "length") (declared-name "length") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))) (name "width") (declared-name "width") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Shell"))) (name "Shell") (declared-name "Shell")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Shell::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Shell")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (name "Sphere") (declared-name "Sphere")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Sphere::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Sphere")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Sphere::radius"))) (name "radius") (declared-name "radius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Sphere")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (name "semiAxis1") (declared-name "semiAxis1") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Sphere")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (name "semiAxis2") (declared-name "semiAxis2") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Sphere")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (name "semiAxis3") (declared-name "semiAxis3") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Sphere")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (name "Tetrahedron") (declared-name "Tetrahedron")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Tetrahedron")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))) (name "baseLength") (declared-name "baseLength") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Tetrahedron")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))) (name "baseWidth") (declared-name "baseWidth") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Tetrahedron")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (name "Toroid") (declared-name "Toroid")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Toroid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Toroid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Toroid::genus"))) (name "genus") (declared-name "genus") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Toroid")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (name "revolutionRadius") (declared-name "revolutionRadius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Toroid")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Torus"))) (name "Torus") (declared-name "Torus")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Torus::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Torus")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))) (name "majorRadius") (declared-name "majorRadius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Torus")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (name "minorRadius") (declared-name "minorRadius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Torus")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (name "Triangle") (declared-name "Triangle")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::Triangle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::Triangle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (name "length") (declared-name "length") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Triangle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Triangle::width"))) (name "width") (declared-name "width") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Triangle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))) (name "xoffset") (declared-name "xoffset") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ShapeItems::Triangle")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (name "TriangularPrism") (declared-name "TriangularPrism")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::TriangularPrism::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ShapeItems::TriangularPrism")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ShapeItems::Wedge"))) (name "Wedge") (declared-name "Wedge"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ShapeItems::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ShapeItems::baseLength"))) (name "baseLength") (declared-name "baseLength") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ShapeItems::baseWidth"))) (name "baseWidth") (declared-name "baseWidth") (declared (properties (ordered false) (unique true))))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::equals"))) (name "equals") (declared-name "equals"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::exists"))) (name "exists") (declared-name "exists"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::forAll"))) (name "forAll") (declared-name "forAll"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::if"))) (name "if") (declared-name "if"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::includes"))) (name "includes") (declared-name "includes"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::m"))) (name "m") (declared-name "m"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::notEmpty"))) (name "notEmpty") (declared-name "notEmpty"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (name "scalarQuantities") (declared-name "scalarQuantities"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ShapeItems::semiMajorAxis"))) (name "semiMajorAxis") (declared-name "semiMajorAxis") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ShapeItems::semiMinorAxis"))) (name "semiMinorAxis") (declared-name "semiMinorAxis") (declared (properties (ordered false) (unique true))))
        (element (kind "import") (id (node (document "d0") (qualified-name "ShapeItems::size"))) (name "size") (declared-name "size"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ShapeItems::xoffset"))) (name "xoffset") (declared-name "xoffset") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 0)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m")))))))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ShapeItems::yoffset"))) (name "yoffset") (declared-name "yoffset") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 0)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m")))))))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Circle::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Circle"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularCone::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::CircularCone"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularDisc::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Cone::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Cone"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::ConicSection::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::ConicSection"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::ConicSurface::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Cuboid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Cuboid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Cylinder::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Cylinder"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Disc::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Disc"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::EccentricCone::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Ellipse::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipse"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Hyperbola::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Line::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Line"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Parabola::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Parabola"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Paraboloid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Path::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Path"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Polygon::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Polygon"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Polyhedron::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Pyramid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Pyramid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Quadrilateral::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Rectangle::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Rectangle"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightTriangle::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Shell::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Shell"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Sphere::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Sphere"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Toroid::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Toroid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Torus::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Torus"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Triangle::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::Triangle"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::TriangularPrism::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::_documentation"))) (to (node (document "d0") (qualified-name "ShapeItems"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (to (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (to (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (to (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))) (to (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Circle"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipse"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (to (node (document "d0") (qualified-name "ShapeItems::Cone"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (to (node (document "d0") (qualified-name "ShapeItems::Cylinder"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (to (node (document "d0") (qualified-name "ShapeItems::Disc"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Cone"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (to (node (document "d0") (qualified-name "ShapeItems::Shell"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (to (node (document "d0") (qualified-name "ShapeItems::Path"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (to (node (document "d0") (qualified-name "ShapeItems::Shell"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (to (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (to (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (to (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Disc"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Disc"))) (to (node (document "d0") (qualified-name "ShapeItems::Shell"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))) (to (node (document "d0") (qualified-name "ShapeItems::Cone"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))) (to (node (document "d0") (qualified-name "ShapeItems::Cylinder"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (to (node (document "d0") (qualified-name "ShapeItems::ConicSection"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (to (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (to (node (document "d0") (qualified-name "ShapeItems::ConicSection"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (to (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Line"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (to (node (document "d0") (qualified-name "ShapeItems::ConicSection"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (to (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (to (node (document "d0") (qualified-name "ShapeItems::Path"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (to (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (to (node (document "d0") (qualified-name "ShapeItems::Shell"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (to (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (to (node (document "d0") (qualified-name "ShapeItems::Polygon"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (to (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (to (node (document "d0") (qualified-name "ShapeItems::Cuboid"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (to (node (document "d0") (qualified-name "ShapeItems::Pyramid"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (to (node (document "d0") (qualified-name "ShapeItems::Toroid"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (to (node (document "d0") (qualified-name "ShapeItems::CircularCone"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (to (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (to (node (document "d0") (qualified-name "ShapeItems::Triangle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (to (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (to (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (to (node (document "d0") (qualified-name "ShapeItems::Pyramid"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (to (node (document "d0") (qualified-name "ShapeItems::Shell"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Torus"))) (to (node (document "d0") (qualified-name "ShapeItems::Toroid"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (to (node (document "d0") (qualified-name "ShapeItems::Polygon"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (to (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/shape_items.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 1) (end 6 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 1) (end 8 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 1) (end 9 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 1) (end 10 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 1) (end 11 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 1) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 1) (end 13 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 1) (end 14 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 1) (end 15 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 1) (end 16 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 1) (end 17 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 1) (end 18 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 1) (end 19 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 1) (end 20 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 1) (end 21 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 1) (end 22 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 24 1) (end 24 275))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 30 2) (end 30 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 32 2) (end 32 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 36 1) (end 36 216))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 42 2) (end 42 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 2) (end 43 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 58 1) (end 58 430))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 1) (end 75 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 1) (end 76 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 1) (end 77 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 1) (end 78 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 79 1) (end 79 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 1) (end 80 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 100 2) (end 100 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 101 2) (end 101 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 112 2) (end 112 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 150 2) (end 150 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 166 2) (end 166 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 167 2) (end 167 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 221 2) (end 221 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 229 1) (end 229 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 242 2) (end 242 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 243 2) (end 243 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 266 2) (end 266 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 287 2) (end 287 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 309 2) (end 309 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 351 2) (end 351 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 391 2) (end 391 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 392 2) (end 392 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 393 2) (end 393 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 395 2) (end 395 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 396 2) (end 396 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 437 2) (end 437 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 469 2) (end 469 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 518 2) (end 518 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 546 2) (end 546 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 556 2) (end 556 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 558 2) (end 558 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 716 2) (end 716 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 717 2) (end 717 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 718 2) (end 718 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 803 2) (end 803 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 804 2) (end 804 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 805 2) (end 805 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 830 2) (end 830 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 831 2) (end 831 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 832 2) (end 832 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 840 2) (end 840 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 871 2) (end 871 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 872 2) (end 872 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 888 2) (end 888 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 889 2) (end 889 30))
      )
    )
  )
)
~~~
