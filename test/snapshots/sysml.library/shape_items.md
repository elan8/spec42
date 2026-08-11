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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "shape_items.md"
    (diagnostics
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
        (range (start 7 16) (end 7 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 16) (end 20 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 16) (end 21 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 16) (end 22 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 24 25) (end 24 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 36 27) (end 36 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 58 27) (end 58 65))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 2) (end 127 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 2) (end 138 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 2) (end 139 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 229 28) (end 229 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 296 2) (end 296 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 297 2) (end 297 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 298 2) (end 298 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 321 2) (end 321 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 332 2) (end 332 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 333 2) (end 333 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 343 2) (end 343 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 361 2) (end 361 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 373 2) (end 373 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 374 2) (end 374 66))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 840 2) (end 840 47))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "238d61b26273402819b82bb7002d568f6054eb812b3fd9009d6df0dc2a7cd74c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ShapeItems"))) (kind "package") (name "ShapeItems") (declared-name "ShapeItems"))
    (element (id (node (document "d0") (qualified-name "ShapeItems::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Box"))) (kind "alias") (name "Box") (declared-name "Box") (parent (node (document "d0") (qualified-name "ShapeItems"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle"))) (kind "item def") (name "Circle") (declared-name "Circle") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Ellipse")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Circle"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "ShapeItems::Circle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Circle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Circle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (kind "item def") (name "CircularCone") (declared-name "CircularCone") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cone")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::CircularCone"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (kind "item def") (name "CircularCylinder") (declared-name "CircularCylinder") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cylinder")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (kind "item def") (name "CircularDisc") (declared-name "CircularDisc") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Disc")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cone"))) (kind "item def") (name "Cone") (declared-name "Cone") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConeOrCylinder")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cone::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Cone"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind "item def") (name "ConeOrCylinder") (declared-name "ConeOrCylinder") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))) (kind "attribute") (name "genus") (declared-name "genus") (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "genus")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))) (kind "attribute") (name "height") (declared-name "height") (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))) (kind "attribute") (name "yoffset") (declared-name "yoffset") (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "yoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (kind "item def") (name "ConicSection") (declared-name "ConicSection") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Path")) (specializes (reference "PlanarCurve")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSection::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::ConicSection"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (kind "item def") (name "ConicSurface") (declared-name "ConicSurface") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))) (kind "attribute") (name "genus") (declared-name "genus") (parent (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "genus")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (kind "item def") (name "Cuboid") (declared-name "Cuboid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CuboidOrTriangularPrism")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cuboid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Cuboid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind "item def") (name "CuboidOrTriangularPrism") (declared-name "CuboidOrTriangularPrism") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Polyhedron")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (kind "item def") (name "Cylinder") (declared-name "Cylinder") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConeOrCylinder")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cylinder::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Cylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Disc"))) (kind "item def") (name "Disc") (declared-name "Disc") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell")) (specializes (reference "PlanarSurface")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Disc::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Disc"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Disc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Disc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))) (kind "item def") (name "EccentricCone") (declared-name "EccentricCone") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cone")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::EccentricCone::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))) (kind "item def") (name "EccentricCylinder") (declared-name "EccentricCylinder") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cylinder")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (kind "item def") (name "Ellipse") (declared-name "Ellipse") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSection")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Ellipse"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (kind "item def") (name "Ellipsoid") (declared-name "Ellipsoid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSurface")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind "attribute") (name "semiAxis1") (declared-name "semiAxis1") (parent (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind "attribute") (name "semiAxis2") (declared-name "semiAxis2") (parent (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind "attribute") (name "semiAxis3") (declared-name "semiAxis3") (parent (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (kind "item def") (name "Hyperbola") (declared-name "Hyperbola") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSection")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind "attribute") (name "conjugateAxis") (declared-name "conjugateAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind "attribute") (name "tranverseAxis") (declared-name "tranverseAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (kind "item def") (name "Hyperboloid") (declared-name "Hyperboloid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSurface")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind "attribute") (name "conjugateAxis") (declared-name "conjugateAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind "attribute") (name "transverseAxis") (declared-name "transverseAxis") (parent (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Item"))) (kind "import") (name "Item") (declared-name "Item") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Items::Item") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Line"))) (kind "item def") (name "Line") (declared-name "Line") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PlanarCurve")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Line::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Line"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (kind "attribute") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "ShapeItems::Line"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (kind "attribute") (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (parent (node (document "d0") (qualified-name "ShapeItems::Line"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "outerSpaceDimension")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::MatesWith"))) (kind "import") (name "MatesWith") (declared-name "MatesWith") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::MatesWith") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (kind "item def") (name "Parabola") (declared-name "Parabola") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSection")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Parabola::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Parabola"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind "attribute") (name "focalDistance") (declared-name "focalDistance") (parent (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (kind "item def") (name "Paraboloid") (declared-name "Paraboloid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSurface")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind "attribute") (name "focalDistance") (declared-name "focalDistance") (parent (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Path"))) (kind "item def") (name "Path") (declared-name "Path") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StructuredSpaceObject::StructuredCurve")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Path::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Path"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (kind "item def") (name "PlanarCurve") (declared-name "PlanarCurve") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Curve")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))) (kind "attribute") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))) (kind "attribute") (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (parent (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "outerSpaceDimension")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (kind "item def") (name "PlanarSurface") (declared-name "PlanarSurface") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Surface")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))) (kind "attribute") (name "area") (declared-name "area") (parent (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "area")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))) (kind "attribute") (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (parent (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "outerSpaceDimension")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (kind "item def") (name "Polygon") (declared-name "Polygon") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Path")) (specializes (reference "PlanarCurve")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polygon::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Polygon"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))) (kind "attribute") (name "isClosed") (declared-name "isClosed") (parent (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isClosed")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (kind "item def") (name "Polyhedron") (declared-name "Polyhedron") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))) (kind "attribute") (name "genus") (declared-name "genus") (parent (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "genus")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))) (kind "attribute") (name "isClosed") (declared-name "isClosed") (parent (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isClosed")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))) (kind "attribute") (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (parent (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "outerSpaceDimension")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (kind "item def") (name "Pyramid") (declared-name "Pyramid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Polyhedron")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::height"))) (kind "attribute") (name "height") (declared-name "height") (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (kind "attribute") (name "wallNumber") (declared-name "wallNumber") (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (authored (membership (kind Feature)) (relationships (typing (reference "Positive")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))) (kind "attribute") (name "yoffset") (declared-name "yoffset") (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "yoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (kind "item def") (name "Quadrilateral") (declared-name "Quadrilateral") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Polygon")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Quadrilateral::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (kind "item def") (name "Rectangle") (declared-name "Rectangle") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Quadrilateral")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Rectangle"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (kind "attribute") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (kind "item def") (name "RectangularCuboid") (declared-name "RectangularCuboid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cuboid")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))) (kind "attribute") (name "height") (declared-name "height") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))) (kind "attribute") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (kind "item def") (name "RectangularPyramid") (declared-name "RectangularPyramid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Pyramid")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))) (kind "attribute") (name "baseLength") (declared-name "baseLength") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseLength")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))) (kind "attribute") (name "baseWidth") (declared-name "baseWidth") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseWidth")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (kind "item def") (name "RectangularToroid") (declared-name "RectangularToroid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Toroid")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind "attribute") (name "rectangleLength") (declared-name "rectangleLength") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind "attribute") (name "rectangleWidth") (declared-name "rectangleWidth") (parent (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (kind "item def") (name "RightCircularCone") (declared-name "RightCircularCone") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CircularCone")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (kind "attribute") (name "yoffset") (declared-name "yoffset") (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "yoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind "item def") (name "RightCircularCylinder") (declared-name "RightCircularCylinder") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CircularCylinder")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (kind "attribute") (name "yoffset") (declared-name "yoffset") (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "yoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (kind "item def") (name "RightTriangle") (declared-name "RightTriangle") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Triangle")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind "item def") (name "RightTriangularPrism") (declared-name "RightTriangularPrism") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TriangularPrism")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))) (kind "attribute") (name "height") (declared-name "height") (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))) (kind "attribute") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Shell"))) (kind "item def") (name "Shell") (declared-name "Shell") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StructuredSpaceObject::StructuredSurface")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Shell::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Shell"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (kind "item def") (name "Sphere") (declared-name "Sphere") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Ellipsoid")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (kind "attribute") (name "semiAxis1") (declared-name "semiAxis1") (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiAxis1")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (kind "attribute") (name "semiAxis2") (declared-name "semiAxis2") (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiAxis2")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (kind "attribute") (name "semiAxis3") (declared-name "semiAxis3") (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiAxis3")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (kind "item def") (name "Tetrahedron") (declared-name "Tetrahedron") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Pyramid")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))) (kind "attribute") (name "baseLength") (declared-name "baseLength") (parent (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseLength")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))) (kind "attribute") (name "baseWidth") (declared-name "baseWidth") (parent (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseWidth")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (kind "item def") (name "Toroid") (declared-name "Toroid") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Toroid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Toroid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Toroid::genus"))) (kind "attribute") (name "genus") (declared-name "genus") (parent (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "genus")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind "attribute") (name "revolutionRadius") (declared-name "revolutionRadius") (parent (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Torus"))) (kind "item def") (name "Torus") (declared-name "Torus") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Toroid")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Torus::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Torus"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind "attribute") (name "majorRadius") (declared-name "majorRadius") (parent (node (document "d0") (qualified-name "ShapeItems::Torus"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "revolutionRadius")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind "attribute") (name "minorRadius") (declared-name "minorRadius") (parent (node (document "d0") (qualified-name "ShapeItems::Torus"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (kind "item def") (name "Triangle") (declared-name "Triangle") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Polygon")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::Triangle"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (kind "attribute") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (parent (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (kind "item def") (name "TriangularPrism") (declared-name "TriangularPrism") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CuboidOrTriangularPrism")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::TriangularPrism::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Wedge"))) (kind "alias") (name "Wedge") (declared-name "Wedge") (parent (node (document "d0") (qualified-name "ShapeItems"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ShapeItems"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::baseLength"))) (kind "attribute def") (name "baseLength") (declared-name "baseLength") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::baseWidth"))) (kind "attribute def") (name "baseWidth") (declared-name "baseWidth") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::equals"))) (kind "import") (name "equals") (declared-name "equals") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::equals") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::exists"))) (kind "import") (name "exists") (declared-name "exists") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::exists") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::if"))) (kind "import") (name "if") (declared-name "if") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::if") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::includes"))) (kind "import") (name "includes") (declared-name "includes") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::m"))) (kind "import") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::m") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (kind "import") (name "scalarQuantities") (declared-name "scalarQuantities") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::scalarQuantities") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::semiMajorAxis"))) (kind "attribute def") (name "semiMajorAxis") (declared-name "semiMajorAxis") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::semiMinorAxis"))) (kind "attribute def") (name "semiMinorAxis") (declared-name "semiMinorAxis") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::xoffset"))) (kind "attribute def") (name "xoffset") (declared-name "xoffset") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::yoffset"))) (kind "attribute def") (name "yoffset") (declared-name "yoffset") (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQSpaceTime::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Objects::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Circle"))) (kind specialization) (ordinal 0)) (authored-target "Ellipse") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Circle::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Circle::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (kind specialization) (ordinal 0)) (authored-target "Cone") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cone")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (kind specialization) (ordinal 0)) (authored-target "Cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (kind specialization) (ordinal 0)) (authored-target "Disc") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Disc")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Cone"))) (kind specialization) (ordinal 0)) (authored-target "ConeOrCylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))) (kind redefinition) (ordinal 0)) (authored-target "genus") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))) (kind redefinition) (ordinal 0)) (authored-target "yoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 0)) (authored-target "Path") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Path")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 1)) (authored-target "PlanarCurve") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))) (kind redefinition) (ordinal 0)) (authored-target "genus") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (kind specialization) (ordinal 0)) (authored-target "CuboidOrTriangularPrism") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind specialization) (ordinal 0)) (authored-target "Polyhedron") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (kind specialization) (ordinal 0)) (authored-target "ConeOrCylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 1)) (authored-target "PlanarSurface") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))) (kind specialization) (ordinal 0)) (authored-target "Cone") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cone")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))) (kind specialization) (ordinal 0)) (authored-target "Cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (kind specialization) (ordinal 0)) (authored-target "ConicSection") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (kind specialization) (ordinal 0)) (authored-target "ConicSurface") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (kind specialization) (ordinal 0)) (authored-target "ConicSection") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (kind specialization) (ordinal 0)) (authored-target "ConicSurface") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Item"))) (kind membershipImport) (ordinal 0)) (authored-target "Items::Item") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Line"))) (kind specialization) (ordinal 0)) (authored-target "PlanarCurve") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Line::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (kind redefinition) (ordinal 0)) (authored-target "outerSpaceDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::MatesWith"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::MatesWith") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (kind specialization) (ordinal 0)) (authored-target "ConicSection") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (kind specialization) (ordinal 0)) (authored-target "ConicSurface") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Path"))) (kind specialization) (ordinal 0)) (authored-target "StructuredSpaceObject::StructuredCurve") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (kind specialization) (ordinal 0)) (authored-target "Curve") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))) (kind redefinition) (ordinal 0)) (authored-target "outerSpaceDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (kind specialization) (ordinal 0)) (authored-target "Surface") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))) (kind redefinition) (ordinal 0)) (authored-target "area") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))) (kind redefinition) (ordinal 0)) (authored-target "outerSpaceDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 0)) (authored-target "Path") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Path")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 1)) (authored-target "PlanarCurve") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))) (kind redefinition) (ordinal 0)) (authored-target "isClosed") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))) (kind redefinition) (ordinal 0)) (authored-target "genus") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))) (kind redefinition) (ordinal 0)) (authored-target "isClosed") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))) (kind redefinition) (ordinal 0)) (authored-target "outerSpaceDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (kind specialization) (ordinal 0)) (authored-target "Polyhedron") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::height")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "Positive") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Positive")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))) (kind redefinition) (ordinal 0)) (authored-target "yoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (kind specialization) (ordinal 0)) (authored-target "Polygon") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polygon")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (kind specialization) (ordinal 0)) (authored-target "Quadrilateral") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Quadrilateral")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Rectangle::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Rectangle::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Rectangle::width")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (kind specialization) (ordinal 0)) (authored-target "Cuboid") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cuboid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (kind specialization) (ordinal 0)) (authored-target "Pyramid") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))) (kind redefinition) (ordinal 0)) (authored-target "baseLength") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))) (kind redefinition) (ordinal 0)) (authored-target "baseWidth") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (kind specialization) (ordinal 0)) (authored-target "Toroid") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Toroid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (kind specialization) (ordinal 0)) (authored-target "CircularCone") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (kind redefinition) (ordinal 0)) (authored-target "yoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind specialization) (ordinal 0)) (authored-target "CircularCylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (kind redefinition) (ordinal 0)) (authored-target "yoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (kind specialization) (ordinal 0)) (authored-target "Triangle") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Triangle")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind specialization) (ordinal 0)) (authored-target "TriangularPrism") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::TriangularPrism")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Shell"))) (kind specialization) (ordinal 0)) (authored-target "StructuredSpaceObject::StructuredSurface") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (kind specialization) (ordinal 0)) (authored-target "Ellipsoid") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Ellipsoid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (kind redefinition) (ordinal 0)) (authored-target "semiAxis1") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (kind redefinition) (ordinal 0)) (authored-target "semiAxis2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (kind redefinition) (ordinal 0)) (authored-target "semiAxis3") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (kind specialization) (ordinal 0)) (authored-target "Pyramid") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))) (kind redefinition) (ordinal 0)) (authored-target "baseLength") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))) (kind redefinition) (ordinal 0)) (authored-target "baseWidth") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Toroid::genus"))) (kind redefinition) (ordinal 0)) (authored-target "genus") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Toroid::genus")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Torus"))) (kind specialization) (ordinal 0)) (authored-target "Toroid") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Toroid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind redefinition) (ordinal 0)) (authored-target "revolutionRadius") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (kind specialization) (ordinal 0)) (authored-target "Polygon") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polygon")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Triangle::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::width")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (kind specialization) (ordinal 0)) (authored-target "CuboidOrTriangularPrism") (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::baseLength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::baseWidth"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::equals"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::equals") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::exists"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::exists") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::if"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::if") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::m"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::m") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::scalarQuantities") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::semiMajorAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::semiMinorAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::xoffset"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::yoffset"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Circle"))) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Circle"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Circle::radius"))) (target (node (document "d0") (qualified-name "ShapeItems::Circle::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Circle::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (target (node (document "d0") (qualified-name "ShapeItems::Cone"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (target (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (target (node (document "d0") (qualified-name "ShapeItems::Disc"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Cone"))) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Cone"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (target (node (document "d0") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (target (node (document "d0") (qualified-name "ShapeItems::Path"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (target (node (document "d0") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (target (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Disc"))) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Disc"))) (target (node (document "d0") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))) (target (node (document "d0") (qualified-name "ShapeItems::Cone"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))) (target (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Line"))) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Line"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (target (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (target (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (target (node (document "d0") (qualified-name "ShapeItems::Path"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))) (target (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (target (node (document "d0") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::height"))) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::height"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::height"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (target (node (document "d0") (qualified-name "ShapeItems::Positive"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (target (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (target (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (target (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Rectangle::width"))) (target (node (document "d0") (qualified-name "ShapeItems::Rectangle::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Rectangle::width"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (target (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))) (target (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))) (target (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (target (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (target (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (target (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (target (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Sphere::radius"))) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Sphere::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))) (target (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))) (target (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (target (node (document "d0") (qualified-name "ShapeItems::Shell"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Toroid::genus"))) (target (node (document "d0") (qualified-name "ShapeItems::Toroid::genus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Toroid::genus"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Torus"))) (target (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Torus"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))) (target (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (target (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Triangle::width"))) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Triangle::width"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (target (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "ShapeItems::xoffset")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "ShapeItems::yoffset")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 42 16) (end 42 20)) (probe (position 42 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))
        (kind redefinition) (ordinal 0) (authored-target "area")
        (range (start 42 16) (end 42 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::PlanarSurface::area") (range (start 42 2) (end 42 25)))
        )
      )
    )
    (query (range (start 82 26) (end 82 30)) (probe (position 82 26))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConicSection"))
        (kind specialization) (ordinal 0) (authored-target "Path")
        (range (start 82 26) (end 82 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Path") (range (start 58 1) (end 58 430)))
        )
      )
    )
    (query (range (start 142 21) (end 142 25)) (probe (position 142 21))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Polygon"))
        (kind specialization) (ordinal 0) (authored-target "Path")
        (range (start 142 21) (end 142 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Path") (range (start 58 1) (end 58 430)))
        )
      )
    )
    (query (range (start 260 26) (end 260 30)) (probe (position 260 26))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularDisc"))
        (kind specialization) (ordinal 0) (authored-target "Disc")
        (range (start 260 26) (end 260 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Disc") (range (start 236 1) (end 236 618)))
        )
      )
    )
    (query (range (start 454 27) (end 454 31)) (probe (position 454 27))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::EccentricCone"))
        (kind specialization) (ordinal 0) (authored-target "Cone")
        (range (start 454 27) (end 454 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Cone") (range (start 440 1) (end 440 280)))
        )
      )
    )
    (query (range (start 463 26) (end 463 30)) (probe (position 463 26))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularCone"))
        (kind specialization) (ordinal 0) (authored-target "Cone")
        (range (start 463 26) (end 463 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Cone") (range (start 440 1) (end 440 280)))
        )
      )
    )
    (query (range (start 10 16) (end 10 21)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::m"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::m")
        (range (start 10 16) (end 10 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 25) (end 24 30)) (probe (position 24 25))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::PlanarCurve"))
        (kind specialization) (ordinal 0) (authored-target "Curve")
        (range (start 24 25) (end 24 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 166 16) (end 166 21)) (probe (position 166 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Triangle::width"))
        (kind redefinition) (ordinal 0) (authored-target "width")
        (range (start 166 16) (end 166 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Triangle::width") (range (start 166 2) (end 166 26)))
        )
      )
    )
    (query (range (start 221 16) (end 221 21)) (probe (position 221 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Rectangle::width"))
        (kind redefinition) (ordinal 0) (authored-target "width")
        (range (start 221 16) (end 221 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Rectangle::width") (range (start 221 2) (end 221 26)))
        )
      )
    )
    (query (range (start 236 18) (end 236 23)) (probe (position 236 18))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Disc"))
        (kind specialization) (ordinal 0) (authored-target "Shell")
        (range (start 236 18) (end 236 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Shell") (range (start 229 1) (end 229 141)))
        )
      )
    )
    (query (range (start 277 26) (end 277 31)) (probe (position 277 26))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConicSurface"))
        (kind specialization) (ordinal 0) (authored-target "Shell")
        (range (start 277 26) (end 277 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Shell") (range (start 229 1) (end 229 141)))
        )
      )
    )
    (query (range (start 287 16) (end 287 21)) (probe (position 287 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))
        (kind redefinition) (ordinal 0) (authored-target "genus")
        (range (start 287 16) (end 287 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConicSurface::genus") (range (start 287 2) (end 287 26)))
        )
      )
    )
    (query (range (start 336 20) (end 336 25)) (probe (position 336 20))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Toroid"))
        (kind specialization) (ordinal 0) (authored-target "Shell")
        (range (start 336 20) (end 336 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Shell") (range (start 229 1) (end 229 141)))
        )
      )
    )
    (query (range (start 351 16) (end 351 21)) (probe (position 351 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Toroid::genus"))
        (kind redefinition) (ordinal 0) (authored-target "genus")
        (range (start 351 16) (end 351 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Toroid::genus") (range (start 351 2) (end 351 26)))
        )
      )
    )
    (query (range (start 383 28) (end 383 33)) (probe (position 383 28))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))
        (kind specialization) (ordinal 0) (authored-target "Shell")
        (range (start 383 28) (end 383 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Shell") (range (start 229 1) (end 229 141)))
        )
      )
    )
    (query (range (start 437 16) (end 437 21)) (probe (position 437 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))
        (kind redefinition) (ordinal 0) (authored-target "genus")
        (range (start 437 16) (end 437 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus") (range (start 437 2) (end 437 26)))
        )
      )
    )
    (query (range (start 540 24) (end 540 29)) (probe (position 540 24))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Polyhedron"))
        (kind specialization) (ordinal 0) (authored-target "Shell")
        (range (start 540 24) (end 540 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Shell") (range (start 229 1) (end 229 141)))
        )
      )
    )
    (query (range (start 558 16) (end 558 21)) (probe (position 558 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))
        (kind redefinition) (ordinal 0) (authored-target "genus")
        (range (start 558 16) (end 558 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Polyhedron::genus") (range (start 558 2) (end 558 26)))
        )
      )
    )
    (query (range (start 717 16) (end 717 21)) (probe (position 717 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))
        (kind redefinition) (ordinal 0) (authored-target "width")
        (range (start 717 16) (end 717 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width") (range (start 717 2) (end 717 26)))
        )
      )
    )
    (query (range (start 804 16) (end 804 21)) (probe (position 804 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))
        (kind redefinition) (ordinal 0) (authored-target "width")
        (range (start 804 16) (end 804 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width") (range (start 804 2) (end 804 26)))
        )
      )
    )
    (query (range (start 30 16) (end 30 22)) (probe (position 30 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))
        (kind redefinition) (ordinal 0) (authored-target "length")
        (range (start 30 16) (end 30 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::PlanarCurve::length") (range (start 30 2) (end 30 27)))
        )
      )
    )
    (query (range (start 54 16) (end 54 22)) (probe (position 54 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Line::length"))
        (kind redefinition) (ordinal 0) (authored-target "length")
        (range (start 54 16) (end 54 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Line::length") (range (start 54 2) (end 54 27)))
        )
      )
    )
    (query (range (start 112 16) (end 112 22)) (probe (position 112 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Circle::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 112 16) (end 112 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Circle::radius") (range (start 112 2) (end 112 27)))
        )
      )
    )
    (query (range (start 165 16) (end 165 22)) (probe (position 165 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Triangle::length"))
        (kind redefinition) (ordinal 0) (authored-target "length")
        (range (start 165 16) (end 165 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Triangle::length") (range (start 165 2) (end 165 27)))
        )
      )
    )
    (query (range (start 220 16) (end 220 22)) (probe (position 220 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Rectangle::length"))
        (kind redefinition) (ordinal 0) (authored-target "length")
        (range (start 220 16) (end 220 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Rectangle::length") (range (start 220 2) (end 220 27)))
        )
      )
    )
    (query (range (start 266 16) (end 266 22)) (probe (position 266 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 266 16) (end 266 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularDisc::radius") (range (start 266 2) (end 266 27)))
        )
      )
    )
    (query (range (start 309 16) (end 309 22)) (probe (position 309 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Sphere::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 309 16) (end 309 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Sphere::radius") (range (start 309 2) (end 309 27)))
        )
      )
    )
    (query (range (start 354 19) (end 354 25)) (probe (position 354 19))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Torus"))
        (kind specialization) (ordinal 0) (authored-target "Toroid")
        (range (start 354 19) (end 354 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Toroid") (range (start 336 1) (end 336 442)))
        )
      )
    )
    (query (range (start 367 31) (end 367 37)) (probe (position 367 31))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularToroid"))
        (kind specialization) (ordinal 0) (authored-target "Toroid")
        (range (start 367 31) (end 367 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Toroid") (range (start 336 1) (end 336 442)))
        )
      )
    )
    (query (range (start 393 16) (end 393 22)) (probe (position 393 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))
        (kind redefinition) (ordinal 0) (authored-target "height")
        (range (start 393 16) (end 393 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height") (range (start 393 2) (end 393 27)))
        )
      )
    )
    (query (range (start 469 16) (end 469 22)) (probe (position 469 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 469 16) (end 469 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularCone::radius") (range (start 469 2) (end 469 27)))
        )
      )
    )
    (query (range (start 518 16) (end 518 22)) (probe (position 518 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 518 16) (end 518 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius") (range (start 518 2) (end 518 27)))
        )
      )
    )
    (query (range (start 716 16) (end 716 22)) (probe (position 716 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))
        (kind redefinition) (ordinal 0) (authored-target "length")
        (range (start 716 16) (end 716 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length") (range (start 716 2) (end 716 27)))
        )
      )
    )
    (query (range (start 718 16) (end 718 22)) (probe (position 718 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))
        (kind redefinition) (ordinal 0) (authored-target "height")
        (range (start 718 16) (end 718 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height") (range (start 718 2) (end 718 27)))
        )
      )
    )
    (query (range (start 797 31) (end 797 37)) (probe (position 797 31))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))
        (kind specialization) (ordinal 0) (authored-target "Cuboid")
        (range (start 797 31) (end 797 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Cuboid") (range (start 745 1) (end 745 1884)))
        )
      )
    )
    (query (range (start 803 16) (end 803 22)) (probe (position 803 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))
        (kind redefinition) (ordinal 0) (authored-target "length")
        (range (start 803 16) (end 803 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length") (range (start 803 2) (end 803 27)))
        )
      )
    )
    (query (range (start 805 16) (end 805 22)) (probe (position 805 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))
        (kind redefinition) (ordinal 0) (authored-target "height")
        (range (start 805 16) (end 805 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height") (range (start 805 2) (end 805 27)))
        )
      )
    )
    (query (range (start 830 16) (end 830 22)) (probe (position 830 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Pyramid::height"))
        (kind redefinition) (ordinal 0) (authored-target "height")
        (range (start 830 16) (end 830 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Pyramid::height") (range (start 830 2) (end 830 27)))
        )
      )
    )
    (query (range (start 9 16) (end 9 23)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQBase::*")
        (range (start 9 16) (end 9 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 23)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "Objects::*")
        (range (start 12 16) (end 12 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 36 27) (end 36 34)) (probe (position 36 27))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::PlanarSurface"))
        (kind specialization) (ordinal 0) (authored-target "Surface")
        (range (start 36 27) (end 36 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 106 20) (end 106 27)) (probe (position 106 20))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Circle"))
        (kind specialization) (ordinal 0) (authored-target "Ellipse")
        (range (start 106 20) (end 106 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Ellipse") (range (start 94 1) (end 94 232)))
        )
      )
    )
    (query (range (start 158 22) (end 158 29)) (probe (position 158 22))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Triangle"))
        (kind specialization) (ordinal 0) (authored-target "Polygon")
        (range (start 158 22) (end 158 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Polygon") (range (start 142 1) (end 142 468)))
        )
      )
    )
    (query (range (start 167 16) (end 167 23)) (probe (position 167 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))
        (kind redefinition) (ordinal 0) (authored-target "xoffset")
        (range (start 167 16) (end 167 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Triangle::xoffset") (range (start 167 2) (end 167 28)))
        )
      )
    )
    (query (range (start 186 16) (end 186 23)) (probe (position 186 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))
        (kind redefinition) (ordinal 0) (authored-target "xoffset")
        (range (start 186 16) (end 186 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset") (range (start 186 2) (end 186 37)))
        )
      )
    )
    (query (range (start 195 27) (end 195 34)) (probe (position 195 27))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Quadrilateral"))
        (kind specialization) (ordinal 0) (authored-target "Polygon")
        (range (start 195 27) (end 195 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Polygon") (range (start 142 1) (end 142 468)))
        )
      )
    )
    (query (range (start 395 16) (end 395 23)) (probe (position 395 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))
        (kind redefinition) (ordinal 0) (authored-target "xoffset")
        (range (start 395 16) (end 395 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset") (range (start 395 2) (end 395 28)))
        )
      )
    )
    (query (range (start 396 16) (end 396 23)) (probe (position 396 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))
        (kind redefinition) (ordinal 0) (authored-target "yoffset")
        (range (start 396 16) (end 396 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset") (range (start 396 2) (end 396 28)))
        )
      )
    )
    (query (range (start 484 16) (end 484 23)) (probe (position 484 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))
        (kind redefinition) (ordinal 0) (authored-target "xoffset")
        (range (start 484 16) (end 484 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset") (range (start 484 2) (end 484 50)))
        )
      )
    )
    (query (range (start 485 16) (end 485 23)) (probe (position 485 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))
        (kind redefinition) (ordinal 0) (authored-target "yoffset")
        (range (start 485 16) (end 485 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset") (range (start 485 2) (end 485 50)))
        )
      )
    )
    (query (range (start 536 16) (end 536 23)) (probe (position 536 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))
        (kind redefinition) (ordinal 0) (authored-target "xoffset")
        (range (start 536 16) (end 536 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset") (range (start 536 2) (end 536 50)))
        )
      )
    )
    (query (range (start 537 16) (end 537 23)) (probe (position 537 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))
        (kind redefinition) (ordinal 0) (authored-target "yoffset")
        (range (start 537 16) (end 537 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset") (range (start 537 2) (end 537 50)))
        )
      )
    )
    (query (range (start 831 16) (end 831 23)) (probe (position 831 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))
        (kind redefinition) (ordinal 0) (authored-target "xoffset")
        (range (start 831 16) (end 831 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset") (range (start 831 2) (end 831 24)))
        )
      )
    )
    (query (range (start 832 16) (end 832 23)) (probe (position 832 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))
        (kind redefinition) (ordinal 0) (authored-target "yoffset")
        (range (start 832 16) (end 832 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset") (range (start 832 2) (end 832 24)))
        )
      )
    )
    (query (range (start 865 25) (end 865 32)) (probe (position 865 25))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Tetrahedron"))
        (kind specialization) (ordinal 0) (authored-target "Pyramid")
        (range (start 865 25) (end 865 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Pyramid") (range (start 822 1) (end 822 1422)))
        )
      )
    )
    (query (range (start 882 32) (end 882 39)) (probe (position 882 32))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))
        (kind specialization) (ordinal 0) (authored-target "Pyramid")
        (range (start 882 32) (end 882 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Pyramid") (range (start 822 1) (end 822 1422)))
        )
      )
    )
    (query (range (start 150 16) (end 150 24)) (probe (position 150 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))
        (kind redefinition) (ordinal 0) (authored-target "isClosed")
        (range (start 150 16) (end 150 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Polygon::isClosed") (range (start 150 2) (end 150 32)))
        )
      )
    )
    (query (range (start 180 27) (end 180 35)) (probe (position 180 27))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightTriangle"))
        (kind specialization) (ordinal 0) (authored-target "Triangle")
        (range (start 180 27) (end 180 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Triangle") (range (start 158 1) (end 158 643)))
        )
      )
    )
    (query (range (start 503 31) (end 503 39)) (probe (position 503 31))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))
        (kind specialization) (ordinal 0) (authored-target "Cylinder")
        (range (start 503 31) (end 503 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Cylinder") (range (start 488 1) (end 488 277)))
        )
      )
    )
    (query (range (start 512 30) (end 512 38)) (probe (position 512 30))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularCylinder"))
        (kind specialization) (ordinal 0) (authored-target "Cylinder")
        (range (start 512 30) (end 512 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Cylinder") (range (start 488 1) (end 488 277)))
        )
      )
    )
    (query (range (start 546 16) (end 546 24)) (probe (position 546 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))
        (kind redefinition) (ordinal 0) (authored-target "isClosed")
        (range (start 546 16) (end 546 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed") (range (start 546 2) (end 546 32)))
        )
      )
    )
    (query (range (start 303 20) (end 303 29)) (probe (position 303 20))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Sphere"))
        (kind specialization) (ordinal 0) (authored-target "Ellipsoid")
        (range (start 303 20) (end 303 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Ellipsoid") (range (start 290 1) (end 290 339)))
        )
      )
    )
    (query (range (start 310 16) (end 310 25)) (probe (position 310 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))
        (kind redefinition) (ordinal 0) (authored-target "semiAxis1")
        (range (start 310 16) (end 310 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1") (range (start 310 2) (end 310 39)))
        )
      )
    )
    (query (range (start 311 16) (end 311 25)) (probe (position 311 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))
        (kind redefinition) (ordinal 0) (authored-target "semiAxis2")
        (range (start 311 16) (end 311 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2") (range (start 311 2) (end 311 39)))
        )
      )
    )
    (query (range (start 312 16) (end 312 25)) (probe (position 312 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))
        (kind redefinition) (ordinal 0) (authored-target "semiAxis3")
        (range (start 312 16) (end 312 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3") (range (start 312 2) (end 312 39)))
        )
      )
    )
    (query (range (start 872 16) (end 872 25)) (probe (position 872 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))
        (kind redefinition) (ordinal 0) (authored-target "baseWidth")
        (range (start 872 16) (end 872 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth") (range (start 872 2) (end 872 30)))
        )
      )
    )
    (query (range (start 889 16) (end 889 25)) (probe (position 889 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))
        (kind redefinition) (ordinal 0) (authored-target "baseWidth")
        (range (start 889 16) (end 889 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth") (range (start 889 2) (end 889 30)))
        )
      )
    )
    (query (range (start 561 37) (end 561 47)) (probe (position 561 37))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))
        (kind specialization) (ordinal 0) (authored-target "Polyhedron")
        (range (start 561 37) (end 561 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Polyhedron") (range (start 540 1) (end 540 568)))
        )
      )
    )
    (query (range (start 822 21) (end 822 31)) (probe (position 822 21))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Pyramid"))
        (kind specialization) (ordinal 0) (authored-target "Polyhedron")
        (range (start 822 21) (end 822 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Polyhedron") (range (start 540 1) (end 540 568)))
        )
      )
    )
    (query (range (start 871 16) (end 871 26)) (probe (position 871 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))
        (kind redefinition) (ordinal 0) (authored-target "baseLength")
        (range (start 871 16) (end 871 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength") (range (start 871 2) (end 871 31)))
        )
      )
    )
    (query (range (start 888 16) (end 888 26)) (probe (position 888 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))
        (kind redefinition) (ordinal 0) (authored-target "baseLength")
        (range (start 888 16) (end 888 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength") (range (start 888 2) (end 888 31)))
        )
      )
    )
    (query (range (start 13 16) (end 13 27)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Item"))
        (kind membershipImport) (ordinal 0) (authored-target "Items::Item")
        (range (start 13 16) (end 13 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 48 18) (end 48 29)) (probe (position 48 18))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Line"))
        (kind specialization) (ordinal 0) (authored-target "PlanarCurve")
        (range (start 48 18) (end 48 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::PlanarCurve") (range (start 24 1) (end 24 275)))
        )
      )
    )
    (query (range (start 82 32) (end 82 43)) (probe (position 82 32))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConicSection"))
        (kind specialization) (ordinal 1) (authored-target "PlanarCurve")
        (range (start 82 32) (end 82 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::PlanarCurve") (range (start 24 1) (end 24 275)))
        )
      )
    )
    (query (range (start 142 27) (end 142 38)) (probe (position 142 27))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Polygon"))
        (kind specialization) (ordinal 1) (authored-target "PlanarCurve")
        (range (start 142 27) (end 142 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::PlanarCurve") (range (start 24 1) (end 24 275)))
        )
      )
    )
    (query (range (start 8 16) (end 8 28)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQSpaceTime::*")
        (range (start 8 16) (end 8 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 94 21) (end 94 33)) (probe (position 94 21))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Ellipse"))
        (kind specialization) (ordinal 0) (authored-target "ConicSection")
        (range (start 94 21) (end 94 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConicSection") (range (start 82 1) (end 82 202)))
        )
      )
    )
    (query (range (start 121 22) (end 121 34)) (probe (position 121 22))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Parabola"))
        (kind specialization) (ordinal 0) (authored-target "ConicSection")
        (range (start 121 22) (end 121 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConicSection") (range (start 82 1) (end 82 202)))
        )
      )
    )
    (query (range (start 132 23) (end 132 35)) (probe (position 132 23))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Hyperbola"))
        (kind specialization) (ordinal 0) (authored-target "ConicSection")
        (range (start 132 23) (end 132 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConicSection") (range (start 82 1) (end 82 202)))
        )
      )
    )
    (query (range (start 290 23) (end 290 35)) (probe (position 290 23))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Ellipsoid"))
        (kind specialization) (ordinal 0) (authored-target "ConicSurface")
        (range (start 290 23) (end 290 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConicSurface") (range (start 277 1) (end 277 226)))
        )
      )
    )
    (query (range (start 315 24) (end 315 36)) (probe (position 315 24))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Paraboloid"))
        (kind specialization) (ordinal 0) (authored-target "ConicSurface")
        (range (start 315 24) (end 315 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConicSurface") (range (start 277 1) (end 277 226)))
        )
      )
    )
    (query (range (start 326 25) (end 326 37)) (probe (position 326 25))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Hyperboloid"))
        (kind specialization) (ordinal 0) (authored-target "ConicSurface")
        (range (start 326 25) (end 326 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConicSurface") (range (start 277 1) (end 277 226)))
        )
      )
    )
    (query (range (start 478 31) (end 478 43)) (probe (position 478 31))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightCircularCone"))
        (kind specialization) (ordinal 0) (authored-target "CircularCone")
        (range (start 478 31) (end 478 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularCone") (range (start 463 1) (end 463 308)))
        )
      )
    )
    (query (range (start 100 16) (end 100 29)) (probe (position 100 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
        (range (start 100 16) (end 100 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis") (range (start 100 2) (end 100 34)))
        )
      )
    )
    (query (range (start 101 16) (end 101 29)) (probe (position 101 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
        (range (start 101 16) (end 101 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis") (range (start 101 2) (end 101 34)))
        )
      )
    )
    (query (range (start 113 16) (end 113 29)) (probe (position 113 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
        (range (start 113 16) (end 113 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis") (range (start 113 2) (end 113 43)))
        )
      )
    )
    (query (range (start 114 16) (end 114 29)) (probe (position 114 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
        (range (start 114 16) (end 114 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis") (range (start 114 2) (end 114 43)))
        )
      )
    )
    (query (range (start 214 23) (end 214 36)) (probe (position 214 23))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Rectangle"))
        (kind specialization) (ordinal 0) (authored-target "Quadrilateral")
        (range (start 214 23) (end 214 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Quadrilateral") (range (start 195 1) (end 195 451)))
        )
      )
    )
    (query (range (start 236 25) (end 236 38)) (probe (position 236 25))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Disc"))
        (kind specialization) (ordinal 1) (authored-target "PlanarSurface")
        (range (start 236 25) (end 236 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::PlanarSurface") (range (start 36 1) (end 36 216)))
        )
      )
    )
    (query (range (start 242 16) (end 242 29)) (probe (position 242 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
        (range (start 242 16) (end 242 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis") (range (start 242 2) (end 242 34)))
        )
      )
    )
    (query (range (start 243 16) (end 243 29)) (probe (position 243 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
        (range (start 243 16) (end 243 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis") (range (start 243 2) (end 243 34)))
        )
      )
    )
    (query (range (start 267 16) (end 267 29)) (probe (position 267 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
        (range (start 267 16) (end 267 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis") (range (start 267 2) (end 267 43)))
        )
      )
    )
    (query (range (start 268 16) (end 268 29)) (probe (position 268 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
        (range (start 268 16) (end 268 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis") (range (start 268 2) (end 268 43)))
        )
      )
    )
    (query (range (start 391 16) (end 391 29)) (probe (position 391 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
        (range (start 391 16) (end 391 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis") (range (start 391 2) (end 391 34)))
        )
      )
    )
    (query (range (start 392 16) (end 392 29)) (probe (position 392 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
        (range (start 392 16) (end 392 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis") (range (start 392 2) (end 392 34)))
        )
      )
    )
    (query (range (start 470 16) (end 470 29)) (probe (position 470 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
        (range (start 470 16) (end 470 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis") (range (start 470 2) (end 470 43)))
        )
      )
    )
    (query (range (start 471 16) (end 471 29)) (probe (position 471 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
        (range (start 471 16) (end 471 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis") (range (start 471 2) (end 471 43)))
        )
      )
    )
    (query (range (start 519 16) (end 519 29)) (probe (position 519 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMajorAxis")
        (range (start 519 16) (end 519 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis") (range (start 519 2) (end 519 43)))
        )
      )
    )
    (query (range (start 520 16) (end 520 29)) (probe (position 520 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))
        (kind redefinition) (ordinal 0) (authored-target "semiMinorAxis")
        (range (start 520 16) (end 520 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis") (range (start 520 2) (end 520 43)))
        )
      )
    )
    (query (range (start 440 18) (end 440 32)) (probe (position 440 18))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Cone"))
        (kind specialization) (ordinal 0) (authored-target "ConeOrCylinder")
        (range (start 440 18) (end 440 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConeOrCylinder") (range (start 383 1) (end 383 2149)))
        )
      )
    )
    (query (range (start 488 22) (end 488 36)) (probe (position 488 22))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Cylinder"))
        (kind specialization) (ordinal 0) (authored-target "ConeOrCylinder")
        (range (start 488 22) (end 488 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::ConeOrCylinder") (range (start 383 1) (end 383 2149)))
        )
      )
    )
    (query (range (start 709 34) (end 709 49)) (probe (position 709 34))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))
        (kind specialization) (ordinal 0) (authored-target "TriangularPrism")
        (range (start 709 34) (end 709 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::TriangularPrism") (range (start 679 1) (end 679 895)))
        )
      )
    )
    (query (range (start 127 47) (end 127 63)) (probe (position 127 47))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 127 47) (end 127 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 138 47) (end 138 63)) (probe (position 138 47))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 138 47) (end 138 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 139 47) (end 139 63)) (probe (position 139 47))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 139 47) (end 139 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 296 43) (end 296 59)) (probe (position 296 43))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 296 43) (end 296 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 297 43) (end 297 59)) (probe (position 297 43))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 297 43) (end 297 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 298 43) (end 298 59)) (probe (position 298 43))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 298 43) (end 298 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 321 47) (end 321 63)) (probe (position 321 47))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 321 47) (end 321 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 332 48) (end 332 64)) (probe (position 332 48))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 332 48) (end 332 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 333 47) (end 333 63)) (probe (position 333 47))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 333 47) (end 333 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 343 50) (end 343 66)) (probe (position 343 50))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 343 50) (end 343 66))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 360 28) (end 360 44)) (probe (position 360 28))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))
        (kind redefinition) (ordinal 0) (authored-target "revolutionRadius")
        (range (start 360 28) (end 360 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius") (range (start 343 2) (end 343 67)))
        )
      )
    )
    (query (range (start 361 45) (end 361 61)) (probe (position 361 45))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 361 45) (end 361 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 373 49) (end 373 65)) (probe (position 373 49))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 373 49) (end 373 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 374 49) (end 374 65)) (probe (position 374 49))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 374 49) (end 374 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::scalarQuantities") (range (start 22 1) (end 22 45)))
        )
      )
    )
    (query (range (start 530 35) (end 530 51)) (probe (position 530 35))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))
        (kind specialization) (ordinal 0) (authored-target "CircularCylinder")
        (range (start 530 35) (end 530 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CircularCylinder") (range (start 512 1) (end 512 432)))
        )
      )
    )
    (query (range (start 32 16) (end 32 35)) (probe (position 32 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))
        (kind redefinition) (ordinal 0) (authored-target "outerSpaceDimension")
        (range (start 32 16) (end 32 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension") (range (start 32 2) (end 32 36)))
        )
      )
    )
    (query (range (start 43 16) (end 43 35)) (probe (position 43 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))
        (kind redefinition) (ordinal 0) (authored-target "outerSpaceDimension")
        (range (start 43 16) (end 43 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension") (range (start 43 2) (end 43 40)))
        )
      )
    )
    (query (range (start 55 16) (end 55 35)) (probe (position 55 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))
        (kind redefinition) (ordinal 0) (authored-target "outerSpaceDimension")
        (range (start 55 16) (end 55 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension") (range (start 55 2) (end 55 40)))
        )
      )
    )
    (query (range (start 556 16) (end 556 35)) (probe (position 556 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))
        (kind redefinition) (ordinal 0) (authored-target "outerSpaceDimension")
        (range (start 556 16) (end 556 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension") (range (start 556 2) (end 556 68)))
        )
      )
    )
    (query (range (start 6 16) (end 6 37)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 6 16) (end 6 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 38)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Positive"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
        (range (start 7 16) (end 7 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 38)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::MatesWith"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::MatesWith")
        (range (start 11 16) (end 11 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 16) (end 19 38)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::if"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::if")
        (range (start 19 16) (end 19 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 39)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 17 16) (end 17 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 679 29) (end 679 52)) (probe (position 679 29))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::TriangularPrism"))
        (kind specialization) (ordinal 0) (authored-target "CuboidOrTriangularPrism")
        (range (start 679 29) (end 679 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism") (range (start 561 1) (end 561 4719)))
        )
      )
    )
    (query (range (start 745 20) (end 745 43)) (probe (position 745 20))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Cuboid"))
        (kind specialization) (ordinal 0) (authored-target "CuboidOrTriangularPrism")
        (range (start 745 20) (end 745 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism") (range (start 561 1) (end 561 4719)))
        )
      )
    )
    (query (range (start 20 16) (end 20 40)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 20 16) (end 20 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 16) (end 21 40)) (probe (position 21 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::exists"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::exists")
        (range (start 21 16) (end 21 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 41)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::equals"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::equals")
        (range (start 14 16) (end 14 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 42)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::isEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
        (range (start 15 16) (end 15 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 43)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::notEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
        (range (start 16 16) (end 16 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 43)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::includes"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
        (range (start 18 16) (end 18 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 16) (end 22 44)) (probe (position 22 16))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::scalarQuantities"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::scalarQuantities")
        (range (start 22 16) (end 22 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 27) (end 58 65)) (probe (position 58 27))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Path"))
        (kind specialization) (ordinal 0) (authored-target "StructuredSpaceObject::StructuredCurve")
        (range (start 58 27) (end 58 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 229 28) (end 229 68)) (probe (position 229 28))
      (reference
        (source (document "d0") (qualified-name "ShapeItems::Shell"))
        (kind specialization) (ordinal 0) (authored-target "StructuredSpaceObject::StructuredSurface")
        (range (start 229 28) (end 229 68))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
