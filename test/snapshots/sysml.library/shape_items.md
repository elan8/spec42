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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "65079e593071bdbd5a95fd437bcff989d823a65ba3fb43c2b1a3e68c1c1b7487") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ShapeItems"))) (kind "package") (name "ShapeItems") (declared-name "ShapeItems") (range (start (line 0) (character 0)) (end (line 0) (character 26661))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 1)) (end (line 8) (character 32))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 28))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 1)) (end (line 9) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 23))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 1)) (end (line 12) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 23))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 6) (character 1)) (end (line 6) (character 38))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Box"))) (kind "alias") (name "Box") (declared-name "Box") (range (start (line 820) (character 1)) (end (line 820) (character 33))) (parent (node (document "d0") (qualified-name "ShapeItems"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle"))) (kind "item def") (name "Circle") (declared-name "Circle") (range (start (line 106) (character 1)) (end (line 106) (character 320))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Ellipse") (range (start (line 106) (character 20)) (end (line 106) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle::_documentation"))) (kind "documentation") (name "") (range (start (line 106) (character 1)) (end (line 106) (character 320))) (parent (node (document "d0") (qualified-name "ShapeItems::Circle"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 112) (character 2)) (end (line 112) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::Circle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 112) (character 16)) (end (line 112) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (range (start (line 113) (character 2)) (end (line 113) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems::Circle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis") (range (start (line 113) (character 16)) (end (line 113) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (range (start (line 114) (character 2)) (end (line 114) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems::Circle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis") (range (start (line 114) (character 16)) (end (line 114) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (kind "item def") (name "CircularCone") (declared-name "CircularCone") (range (start (line 463) (character 1)) (end (line 463) (character 308))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cone") (range (start (line 463) (character 26)) (end (line 463) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::_documentation"))) (kind "documentation") (name "") (range (start (line 463) (character 1)) (end (line 463) (character 308))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularCone"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 469) (character 2)) (end (line 469) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 469) (character 16)) (end (line 469) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (range (start (line 470) (character 2)) (end (line 470) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis") (range (start (line 470) (character 16)) (end (line 470) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (range (start (line 471) (character 2)) (end (line 471) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis") (range (start (line 471) (character 16)) (end (line 471) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (kind "item def") (name "CircularCylinder") (declared-name "CircularCylinder") (range (start (line 512) (character 1)) (end (line 512) (character 432))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cylinder") (range (start (line 512) (character 30)) (end (line 512) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::_documentation"))) (kind "documentation") (name "") (range (start (line 512) (character 1)) (end (line 512) (character 432))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 518) (character 2)) (end (line 518) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 518) (character 16)) (end (line 518) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (range (start (line 519) (character 2)) (end (line 519) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis") (range (start (line 519) (character 16)) (end (line 519) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (range (start (line 520) (character 2)) (end (line 520) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis") (range (start (line 520) (character 16)) (end (line 520) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (kind "item def") (name "CircularDisc") (declared-name "CircularDisc") (range (start (line 260) (character 1)) (end (line 260) (character 439))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Disc") (range (start (line 260) (character 26)) (end (line 260) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::_documentation"))) (kind "documentation") (name "") (range (start (line 260) (character 1)) (end (line 260) (character 439))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 266) (character 2)) (end (line 266) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 266) (character 16)) (end (line 266) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (range (start (line 267) (character 2)) (end (line 267) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis") (range (start (line 267) (character 16)) (end (line 267) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (range (start (line 268) (character 2)) (end (line 268) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis") (range (start (line 268) (character 16)) (end (line 268) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cone"))) (kind "item def") (name "Cone") (declared-name "Cone") (range (start (line 440) (character 1)) (end (line 440) (character 280))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConeOrCylinder") (range (start (line 440) (character 18)) (end (line 440) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cone::_documentation"))) (kind "documentation") (name "") (range (start (line 440) (character 1)) (end (line 440) (character 280))) (parent (node (document "d0") (qualified-name "ShapeItems::Cone"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind "item def") (name "ConeOrCylinder") (declared-name "ConeOrCylinder") (range (start (line 383) (character 1)) (end (line 383) (character 2149))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell") (range (start (line 383) (character 28)) (end (line 383) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::_documentation"))) (kind "documentation") (name "") (range (start (line 383) (character 1)) (end (line 383) (character 2149))) (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))) (kind "attribute") (name "genus") (declared-name "genus") (range (start (line 437) (character 2)) (end (line 437) (character 26))) (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "genus") (range (start (line 437) (character 16)) (end (line 437) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 393) (character 2)) (end (line 393) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height") (range (start (line 393) (character 16)) (end (line 393) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (range (start (line 391) (character 2)) (end (line 391) (character 34))) (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis") (range (start (line 391) (character 16)) (end (line 391) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (range (start (line 392) (character 2)) (end (line 392) (character 34))) (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis") (range (start (line 392) (character 16)) (end (line 392) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (range (start (line 395) (character 2)) (end (line 395) (character 28))) (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset") (range (start (line 395) (character 16)) (end (line 395) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))) (kind "attribute") (name "yoffset") (declared-name "yoffset") (range (start (line 396) (character 2)) (end (line 396) (character 28))) (parent (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "yoffset") (range (start (line 396) (character 16)) (end (line 396) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (kind "item def") (name "ConicSection") (declared-name "ConicSection") (range (start (line 82) (character 1)) (end (line 82) (character 202))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Path") (range (start (line 82) (character 26)) (end (line 82) (character 30)))) (specializes (reference "PlanarCurve") (range (start (line 82) (character 32)) (end (line 82) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSection::_documentation"))) (kind "documentation") (name "") (range (start (line 82) (character 1)) (end (line 82) (character 202))) (parent (node (document "d0") (qualified-name "ShapeItems::ConicSection"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (kind "item def") (name "ConicSurface") (declared-name "ConicSurface") (range (start (line 277) (character 1)) (end (line 277) (character 226))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell") (range (start (line 277) (character 26)) (end (line 277) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface::_documentation"))) (kind "documentation") (name "") (range (start (line 277) (character 1)) (end (line 277) (character 226))) (parent (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))) (kind "attribute") (name "genus") (declared-name "genus") (range (start (line 287) (character 2)) (end (line 287) (character 26))) (parent (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "genus") (range (start (line 287) (character 16)) (end (line 287) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (kind "item def") (name "Cuboid") (declared-name "Cuboid") (range (start (line 745) (character 1)) (end (line 745) (character 1884))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CuboidOrTriangularPrism") (range (start (line 745) (character 20)) (end (line 745) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cuboid::_documentation"))) (kind "documentation") (name "") (range (start (line 745) (character 1)) (end (line 745) (character 1884))) (parent (node (document "d0") (qualified-name "ShapeItems::Cuboid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind "item def") (name "CuboidOrTriangularPrism") (declared-name "CuboidOrTriangularPrism") (range (start (line 561) (character 1)) (end (line 561) (character 4719))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Polyhedron") (range (start (line 561) (character 37)) (end (line 561) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism::_documentation"))) (kind "documentation") (name "") (range (start (line 561) (character 1)) (end (line 561) (character 4719))) (parent (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (kind "item def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 488) (character 1)) (end (line 488) (character 277))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConeOrCylinder") (range (start (line 488) (character 22)) (end (line 488) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Cylinder::_documentation"))) (kind "documentation") (name "") (range (start (line 488) (character 1)) (end (line 488) (character 277))) (parent (node (document "d0") (qualified-name "ShapeItems::Cylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Disc"))) (kind "item def") (name "Disc") (declared-name "Disc") (range (start (line 236) (character 1)) (end (line 236) (character 618))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell") (range (start (line 236) (character 18)) (end (line 236) (character 23)))) (specializes (reference "PlanarSurface") (range (start (line 236) (character 25)) (end (line 236) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Disc::_documentation"))) (kind "documentation") (name "") (range (start (line 236) (character 1)) (end (line 236) (character 618))) (parent (node (document "d0") (qualified-name "ShapeItems::Disc"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (range (start (line 242) (character 2)) (end (line 242) (character 34))) (parent (node (document "d0") (qualified-name "ShapeItems::Disc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis") (range (start (line 242) (character 16)) (end (line 242) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (range (start (line 243) (character 2)) (end (line 243) (character 34))) (parent (node (document "d0") (qualified-name "ShapeItems::Disc"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis") (range (start (line 243) (character 16)) (end (line 243) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))) (kind "item def") (name "EccentricCone") (declared-name "EccentricCone") (range (start (line 454) (character 1)) (end (line 454) (character 170))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cone") (range (start (line 454) (character 27)) (end (line 454) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::EccentricCone::_documentation"))) (kind "documentation") (name "") (range (start (line 454) (character 1)) (end (line 454) (character 170))) (parent (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))) (kind "item def") (name "EccentricCylinder") (declared-name "EccentricCylinder") (range (start (line 503) (character 1)) (end (line 503) (character 182))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cylinder") (range (start (line 503) (character 31)) (end (line 503) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder::_documentation"))) (kind "documentation") (name "") (range (start (line 503) (character 1)) (end (line 503) (character 182))) (parent (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (kind "item def") (name "Ellipse") (declared-name "Ellipse") (range (start (line 94) (character 1)) (end (line 94) (character 232))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSection") (range (start (line 94) (character 21)) (end (line 94) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::_documentation"))) (kind "documentation") (name "") (range (start (line 94) (character 1)) (end (line 94) (character 232))) (parent (node (document "d0") (qualified-name "ShapeItems::Ellipse"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))) (kind "attribute") (name "semiMajorAxis") (declared-name "semiMajorAxis") (range (start (line 100) (character 2)) (end (line 100) (character 34))) (parent (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMajorAxis") (range (start (line 100) (character 16)) (end (line 100) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))) (kind "attribute") (name "semiMinorAxis") (declared-name "semiMinorAxis") (range (start (line 101) (character 2)) (end (line 101) (character 34))) (parent (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiMinorAxis") (range (start (line 101) (character 16)) (end (line 101) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (kind "item def") (name "Ellipsoid") (declared-name "Ellipsoid") (range (start (line 290) (character 1)) (end (line 290) (character 339))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSurface") (range (start (line 290) (character 23)) (end (line 290) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::_documentation"))) (kind "documentation") (name "") (range (start (line 290) (character 1)) (end (line 290) (character 339))) (parent (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind "attribute") (name "semiAxis1") (declared-name "semiAxis1") (range (start (line 296) (character 2)) (end (line 296) (character 60))) (parent (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 296) (character 43)) (end (line 296) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind "attribute") (name "semiAxis2") (declared-name "semiAxis2") (range (start (line 297) (character 2)) (end (line 297) (character 60))) (parent (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 297) (character 43)) (end (line 297) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind "attribute") (name "semiAxis3") (declared-name "semiAxis3") (range (start (line 298) (character 2)) (end (line 298) (character 60))) (parent (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 298) (character 43)) (end (line 298) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (kind "item def") (name "Hyperbola") (declared-name "Hyperbola") (range (start (line 132) (character 1)) (end (line 132) (character 269))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSection") (range (start (line 132) (character 23)) (end (line 132) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::_documentation"))) (kind "documentation") (name "") (range (start (line 132) (character 1)) (end (line 132) (character 269))) (parent (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind "attribute") (name "conjugateAxis") (declared-name "conjugateAxis") (range (start (line 139) (character 2)) (end (line 139) (character 64))) (parent (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 139) (character 47)) (end (line 139) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind "attribute") (name "tranverseAxis") (declared-name "tranverseAxis") (range (start (line 138) (character 2)) (end (line 138) (character 64))) (parent (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 138) (character 47)) (end (line 138) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (kind "item def") (name "Hyperboloid") (declared-name "Hyperboloid") (range (start (line 326) (character 1)) (end (line 326) (character 266))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSurface") (range (start (line 326) (character 25)) (end (line 326) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::_documentation"))) (kind "documentation") (name "") (range (start (line 326) (character 1)) (end (line 326) (character 266))) (parent (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind "attribute") (name "conjugateAxis") (declared-name "conjugateAxis") (range (start (line 333) (character 2)) (end (line 333) (character 64))) (parent (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 333) (character 47)) (end (line 333) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind "attribute") (name "transverseAxis") (declared-name "transverseAxis") (range (start (line 332) (character 2)) (end (line 332) (character 65))) (parent (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 332) (character 48)) (end (line 332) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Item"))) (kind "import") (name "Item") (declared-name "Item") (range (start (line 13) (character 1)) (end (line 13) (character 28))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Items::Item") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 27))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Line"))) (kind "item def") (name "Line") (declared-name "Line") (range (start (line 48) (character 1)) (end (line 48) (character 188))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PlanarCurve") (range (start (line 48) (character 18)) (end (line 48) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Line::_documentation"))) (kind "documentation") (name "") (range (start (line 48) (character 1)) (end (line 48) (character 188))) (parent (node (document "d0") (qualified-name "ShapeItems::Line"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 54) (character 2)) (end (line 54) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::Line"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length") (range (start (line 54) (character 16)) (end (line 54) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (kind "attribute") (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (range (start (line 55) (character 2)) (end (line 55) (character 40))) (parent (node (document "d0") (qualified-name "ShapeItems::Line"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "outerSpaceDimension") (range (start (line 55) (character 16)) (end (line 55) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::MatesWith"))) (kind "import") (name "MatesWith") (declared-name "MatesWith") (range (start (line 11) (character 1)) (end (line 11) (character 39))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::MatesWith") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 38))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (kind "item def") (name "Parabola") (declared-name "Parabola") (range (start (line 121) (character 1)) (end (line 121) (character 232))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSection") (range (start (line 121) (character 22)) (end (line 121) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Parabola::_documentation"))) (kind "documentation") (name "") (range (start (line 121) (character 1)) (end (line 121) (character 232))) (parent (node (document "d0") (qualified-name "ShapeItems::Parabola"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind "attribute") (name "focalDistance") (declared-name "focalDistance") (range (start (line 127) (character 2)) (end (line 127) (character 64))) (parent (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 127) (character 47)) (end (line 127) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (kind "item def") (name "Paraboloid") (declared-name "Paraboloid") (range (start (line 315) (character 1)) (end (line 315) (character 220))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConicSurface") (range (start (line 315) (character 24)) (end (line 315) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid::_documentation"))) (kind "documentation") (name "") (range (start (line 315) (character 1)) (end (line 315) (character 220))) (parent (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind "attribute") (name "focalDistance") (declared-name "focalDistance") (range (start (line 321) (character 2)) (end (line 321) (character 64))) (parent (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 321) (character 47)) (end (line 321) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Path"))) (kind "item def") (name "Path") (declared-name "Path") (range (start (line 58) (character 1)) (end (line 58) (character 430))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StructuredSpaceObject::StructuredCurve") (range (start (line 58) (character 27)) (end (line 58) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Path::_documentation"))) (kind "documentation") (name "") (range (start (line 58) (character 1)) (end (line 58) (character 430))) (parent (node (document "d0") (qualified-name "ShapeItems::Path"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (kind "item def") (name "PlanarCurve") (declared-name "PlanarCurve") (range (start (line 24) (character 1)) (end (line 24) (character 275))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Curve") (range (start (line 24) (character 25)) (end (line 24) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::_documentation"))) (kind "documentation") (name "") (range (start (line 24) (character 1)) (end (line 24) (character 275))) (parent (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 30) (character 2)) (end (line 30) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length") (range (start (line 30) (character 16)) (end (line 30) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))) (kind "attribute") (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (range (start (line 32) (character 2)) (end (line 32) (character 36))) (parent (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "outerSpaceDimension") (range (start (line 32) (character 16)) (end (line 32) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (kind "item def") (name "PlanarSurface") (declared-name "PlanarSurface") (range (start (line 36) (character 1)) (end (line 36) (character 216))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Surface") (range (start (line 36) (character 27)) (end (line 36) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::_documentation"))) (kind "documentation") (name "") (range (start (line 36) (character 1)) (end (line 36) (character 216))) (parent (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))) (kind "attribute") (name "area") (declared-name "area") (range (start (line 42) (character 2)) (end (line 42) (character 25))) (parent (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "area") (range (start (line 42) (character 16)) (end (line 42) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))) (kind "attribute") (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (range (start (line 43) (character 2)) (end (line 43) (character 40))) (parent (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "outerSpaceDimension") (range (start (line 43) (character 16)) (end (line 43) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (kind "item def") (name "Polygon") (declared-name "Polygon") (range (start (line 142) (character 1)) (end (line 142) (character 468))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Path") (range (start (line 142) (character 21)) (end (line 142) (character 25)))) (specializes (reference "PlanarCurve") (range (start (line 142) (character 27)) (end (line 142) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polygon::_documentation"))) (kind "documentation") (name "") (range (start (line 142) (character 1)) (end (line 142) (character 468))) (parent (node (document "d0") (qualified-name "ShapeItems::Polygon"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))) (kind "attribute") (name "isClosed") (declared-name "isClosed") (range (start (line 150) (character 2)) (end (line 150) (character 32))) (parent (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isClosed") (range (start (line 150) (character 16)) (end (line 150) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (kind "item def") (name "Polyhedron") (declared-name "Polyhedron") (range (start (line 540) (character 1)) (end (line 540) (character 568))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell") (range (start (line 540) (character 24)) (end (line 540) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::_documentation"))) (kind "documentation") (name "") (range (start (line 540) (character 1)) (end (line 540) (character 568))) (parent (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))) (kind "attribute") (name "genus") (declared-name "genus") (range (start (line 558) (character 2)) (end (line 558) (character 26))) (parent (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "genus") (range (start (line 558) (character 16)) (end (line 558) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))) (kind "attribute") (name "isClosed") (declared-name "isClosed") (range (start (line 546) (character 2)) (end (line 546) (character 32))) (parent (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isClosed") (range (start (line 546) (character 16)) (end (line 546) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))) (kind "attribute") (name "outerSpaceDimension") (declared-name "outerSpaceDimension") (range (start (line 556) (character 2)) (end (line 556) (character 68))) (parent (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "outerSpaceDimension") (range (start (line 556) (character 16)) (end (line 556) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (range (start (line 7) (character 1)) (end (line 7) (character 39))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 38))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (kind "item def") (name "Pyramid") (declared-name "Pyramid") (range (start (line 822) (character 1)) (end (line 822) (character 1422))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Polyhedron") (range (start (line 822) (character 21)) (end (line 822) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::_documentation"))) (kind "documentation") (name "") (range (start (line 822) (character 1)) (end (line 822) (character 1422))) (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 830) (character 2)) (end (line 830) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height") (range (start (line 830) (character 16)) (end (line 830) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (kind "attribute") (name "wallNumber") (declared-name "wallNumber") (range (start (line 840) (character 2)) (end (line 840) (character 47))) (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (authored (membership (kind Feature)) (relationships (typing (reference "Positive") (range none)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (range (start (line 831) (character 2)) (end (line 831) (character 24))) (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset") (range (start (line 831) (character 16)) (end (line 831) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))) (kind "attribute") (name "yoffset") (declared-name "yoffset") (range (start (line 832) (character 2)) (end (line 832) (character 24))) (parent (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "yoffset") (range (start (line 832) (character 16)) (end (line 832) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (kind "item def") (name "Quadrilateral") (declared-name "Quadrilateral") (range (start (line 195) (character 1)) (end (line 195) (character 451))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Polygon") (range (start (line 195) (character 27)) (end (line 195) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Quadrilateral::_documentation"))) (kind "documentation") (name "") (range (start (line 195) (character 1)) (end (line 195) (character 451))) (parent (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (kind "item def") (name "Rectangle") (declared-name "Rectangle") (range (start (line 214) (character 1)) (end (line 214) (character 421))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Quadrilateral") (range (start (line 214) (character 23)) (end (line 214) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::_documentation"))) (kind "documentation") (name "") (range (start (line 214) (character 1)) (end (line 214) (character 421))) (parent (node (document "d0") (qualified-name "ShapeItems::Rectangle"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 220) (character 2)) (end (line 220) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length") (range (start (line 220) (character 16)) (end (line 220) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Rectangle::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 221) (character 2)) (end (line 221) (character 26))) (parent (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width") (range (start (line 221) (character 16)) (end (line 221) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (kind "item def") (name "RectangularCuboid") (declared-name "RectangularCuboid") (range (start (line 797) (character 1)) (end (line 797) (character 1053))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Cuboid") (range (start (line 797) (character 31)) (end (line 797) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::_documentation"))) (kind "documentation") (name "") (range (start (line 797) (character 1)) (end (line 797) (character 1053))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 805) (character 2)) (end (line 805) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height") (range (start (line 805) (character 16)) (end (line 805) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 803) (character 2)) (end (line 803) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length") (range (start (line 803) (character 16)) (end (line 803) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 804) (character 2)) (end (line 804) (character 26))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width") (range (start (line 804) (character 16)) (end (line 804) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (kind "item def") (name "RectangularPyramid") (declared-name "RectangularPyramid") (range (start (line 882) (character 1)) (end (line 882) (character 491))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Pyramid") (range (start (line 882) (character 32)) (end (line 882) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::_documentation"))) (kind "documentation") (name "") (range (start (line 882) (character 1)) (end (line 882) (character 491))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))) (kind "attribute") (name "baseLength") (declared-name "baseLength") (range (start (line 888) (character 2)) (end (line 888) (character 31))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseLength") (range (start (line 888) (character 16)) (end (line 888) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))) (kind "attribute") (name "baseWidth") (declared-name "baseWidth") (range (start (line 889) (character 2)) (end (line 889) (character 30))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseWidth") (range (start (line 889) (character 16)) (end (line 889) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (kind "item def") (name "RectangularToroid") (declared-name "RectangularToroid") (range (start (line 367) (character 1)) (end (line 367) (character 447))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Toroid") (range (start (line 367) (character 31)) (end (line 367) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::_documentation"))) (kind "documentation") (name "") (range (start (line 367) (character 1)) (end (line 367) (character 447))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind "attribute") (name "rectangleLength") (declared-name "rectangleLength") (range (start (line 373) (character 2)) (end (line 373) (character 66))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 373) (character 49)) (end (line 373) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind "attribute") (name "rectangleWidth") (declared-name "rectangleWidth") (range (start (line 374) (character 2)) (end (line 374) (character 66))) (parent (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 374) (character 49)) (end (line 374) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (kind "item def") (name "RightCircularCone") (declared-name "RightCircularCone") (range (start (line 478) (character 1)) (end (line 478) (character 231))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CircularCone") (range (start (line 478) (character 31)) (end (line 478) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::_documentation"))) (kind "documentation") (name "") (range (start (line 478) (character 1)) (end (line 478) (character 231))) (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (range (start (line 484) (character 2)) (end (line 484) (character 50))) (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset") (range (start (line 484) (character 16)) (end (line 484) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (kind "attribute") (name "yoffset") (declared-name "yoffset") (range (start (line 485) (character 2)) (end (line 485) (character 50))) (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "yoffset") (range (start (line 485) (character 16)) (end (line 485) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind "item def") (name "RightCircularCylinder") (declared-name "RightCircularCylinder") (range (start (line 530) (character 1)) (end (line 530) (character 247))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CircularCylinder") (range (start (line 530) (character 35)) (end (line 530) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::_documentation"))) (kind "documentation") (name "") (range (start (line 530) (character 1)) (end (line 530) (character 247))) (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (range (start (line 536) (character 2)) (end (line 536) (character 50))) (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset") (range (start (line 536) (character 16)) (end (line 536) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (kind "attribute") (name "yoffset") (declared-name "yoffset") (range (start (line 537) (character 2)) (end (line 537) (character 50))) (parent (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "yoffset") (range (start (line 537) (character 16)) (end (line 537) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (kind "item def") (name "RightTriangle") (declared-name "RightTriangle") (range (start (line 180) (character 1)) (end (line 180) (character 345))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Triangle") (range (start (line 180) (character 27)) (end (line 180) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle::_documentation"))) (kind "documentation") (name "") (range (start (line 180) (character 1)) (end (line 180) (character 345))) (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (range (start (line 186) (character 2)) (end (line 186) (character 37))) (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset") (range (start (line 186) (character 16)) (end (line 186) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind "item def") (name "RightTriangularPrism") (declared-name "RightTriangularPrism") (range (start (line 709) (character 1)) (end (line 709) (character 1169))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TriangularPrism") (range (start (line 709) (character 34)) (end (line 709) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::_documentation"))) (kind "documentation") (name "") (range (start (line 709) (character 1)) (end (line 709) (character 1169))) (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 718) (character 2)) (end (line 718) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height") (range (start (line 718) (character 16)) (end (line 718) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 716) (character 2)) (end (line 716) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length") (range (start (line 716) (character 16)) (end (line 716) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 717) (character 2)) (end (line 717) (character 26))) (parent (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width") (range (start (line 717) (character 16)) (end (line 717) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Shell"))) (kind "item def") (name "Shell") (declared-name "Shell") (range (start (line 229) (character 1)) (end (line 229) (character 141))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StructuredSpaceObject::StructuredSurface") (range (start (line 229) (character 28)) (end (line 229) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Shell::_documentation"))) (kind "documentation") (name "") (range (start (line 229) (character 1)) (end (line 229) (character 141))) (parent (node (document "d0") (qualified-name "ShapeItems::Shell"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (kind "item def") (name "Sphere") (declared-name "Sphere") (range (start (line 303) (character 1)) (end (line 303) (character 259))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Ellipsoid") (range (start (line 303) (character 20)) (end (line 303) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::_documentation"))) (kind "documentation") (name "") (range (start (line 303) (character 1)) (end (line 303) (character 259))) (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 309) (character 2)) (end (line 309) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 309) (character 16)) (end (line 309) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (kind "attribute") (name "semiAxis1") (declared-name "semiAxis1") (range (start (line 310) (character 2)) (end (line 310) (character 39))) (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiAxis1") (range (start (line 310) (character 16)) (end (line 310) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (kind "attribute") (name "semiAxis2") (declared-name "semiAxis2") (range (start (line 311) (character 2)) (end (line 311) (character 39))) (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiAxis2") (range (start (line 311) (character 16)) (end (line 311) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (kind "attribute") (name "semiAxis3") (declared-name "semiAxis3") (range (start (line 312) (character 2)) (end (line 312) (character 39))) (parent (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "semiAxis3") (range (start (line 312) (character 16)) (end (line 312) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (kind "item def") (name "Tetrahedron") (declared-name "Tetrahedron") (range (start (line 865) (character 1)) (end (line 865) (character 460))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Pyramid") (range (start (line 865) (character 25)) (end (line 865) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::_documentation"))) (kind "documentation") (name "") (range (start (line 865) (character 1)) (end (line 865) (character 460))) (parent (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))) (kind "attribute") (name "baseLength") (declared-name "baseLength") (range (start (line 871) (character 2)) (end (line 871) (character 31))) (parent (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseLength") (range (start (line 871) (character 16)) (end (line 871) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))) (kind "attribute") (name "baseWidth") (declared-name "baseWidth") (range (start (line 872) (character 2)) (end (line 872) (character 30))) (parent (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseWidth") (range (start (line 872) (character 16)) (end (line 872) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (kind "item def") (name "Toroid") (declared-name "Toroid") (range (start (line 336) (character 1)) (end (line 336) (character 442))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Shell") (range (start (line 336) (character 20)) (end (line 336) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Toroid::_documentation"))) (kind "documentation") (name "") (range (start (line 336) (character 1)) (end (line 336) (character 442))) (parent (node (document "d0") (qualified-name "ShapeItems::Toroid"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Toroid::genus"))) (kind "attribute") (name "genus") (declared-name "genus") (range (start (line 351) (character 2)) (end (line 351) (character 26))) (parent (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "genus") (range (start (line 351) (character 16)) (end (line 351) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind "attribute") (name "revolutionRadius") (declared-name "revolutionRadius") (range (start (line 343) (character 2)) (end (line 343) (character 67))) (parent (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 343) (character 50)) (end (line 343) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Torus"))) (kind "item def") (name "Torus") (declared-name "Torus") (range (start (line 354) (character 1)) (end (line 354) (character 278))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Toroid") (range (start (line 354) (character 19)) (end (line 354) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Torus::_documentation"))) (kind "documentation") (name "") (range (start (line 354) (character 1)) (end (line 354) (character 278))) (parent (node (document "d0") (qualified-name "ShapeItems::Torus"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind "attribute") (name "majorRadius") (declared-name "majorRadius") (range (start (line 360) (character 2)) (end (line 360) (character 45))) (parent (node (document "d0") (qualified-name "ShapeItems::Torus"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "revolutionRadius") (range (start (line 360) (character 28)) (end (line 360) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind "attribute") (name "minorRadius") (declared-name "minorRadius") (range (start (line 361) (character 2)) (end (line 361) (character 62))) (parent (node (document "d0") (qualified-name "ShapeItems::Torus"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 361) (character 45)) (end (line 361) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (kind "item def") (name "Triangle") (declared-name "Triangle") (range (start (line 158) (character 1)) (end (line 158) (character 643))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Polygon") (range (start (line 158) (character 22)) (end (line 158) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle::_documentation"))) (kind "documentation") (name "") (range (start (line 158) (character 1)) (end (line 158) (character 643))) (parent (node (document "d0") (qualified-name "ShapeItems::Triangle"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 165) (character 2)) (end (line 165) (character 27))) (parent (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length") (range (start (line 165) (character 16)) (end (line 165) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 166) (character 2)) (end (line 166) (character 26))) (parent (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width") (range (start (line 166) (character 16)) (end (line 166) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))) (kind "attribute") (name "xoffset") (declared-name "xoffset") (range (start (line 167) (character 2)) (end (line 167) (character 28))) (parent (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "xoffset") (range (start (line 167) (character 16)) (end (line 167) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (kind "item def") (name "TriangularPrism") (declared-name "TriangularPrism") (range (start (line 679) (character 1)) (end (line 679) (character 895))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CuboidOrTriangularPrism") (range (start (line 679) (character 29)) (end (line 679) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::TriangularPrism::_documentation"))) (kind "documentation") (name "") (range (start (line 679) (character 1)) (end (line 679) (character 895))) (parent (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::Wedge"))) (kind "alias") (name "Wedge") (declared-name "Wedge") (range (start (line 743) (character 1)) (end (line 743) (character 38))) (parent (node (document "d0") (qualified-name "ShapeItems"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 26661))) (parent (node (document "d0") (qualified-name "ShapeItems"))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::baseLength"))) (kind "attribute def") (name "baseLength") (declared-name "baseLength") (range (start (line 79) (character 1)) (end (line 79) (character 63))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::baseWidth"))) (kind "attribute def") (name "baseWidth") (declared-name "baseWidth") (range (start (line 80) (character 1)) (end (line 80) (character 62))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::equals"))) (kind "import") (name "equals") (declared-name "equals") (range (start (line 14) (character 1)) (end (line 14) (character 42))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::equals") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 41))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::exists"))) (kind "import") (name "exists") (declared-name "exists") (range (start (line 21) (character 1)) (end (line 21) (character 41))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::exists") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 21) (character 16)) (end (line 21) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 20) (character 1)) (end (line 20) (character 41))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 16)) (end (line 20) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::if"))) (kind "import") (name "if") (declared-name "if") (range (start (line 19) (character 1)) (end (line 19) (character 39))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::if") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 38))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::includes"))) (kind "import") (name "includes") (declared-name "includes") (range (start (line 18) (character 1)) (end (line 18) (character 44))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 43))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (range (start (line 15) (character 1)) (end (line 15) (character 43))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 42))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::m"))) (kind "import") (name "m") (declared-name "m") (range (start (line 10) (character 1)) (end (line 10) (character 22))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::m") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 21))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (range (start (line 16) (character 1)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 43))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (kind "import") (name "scalarQuantities") (declared-name "scalarQuantities") (range (start (line 22) (character 1)) (end (line 22) (character 45))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::scalarQuantities") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 22) (character 16)) (end (line 22) (character 44))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::semiMajorAxis"))) (kind "attribute def") (name "semiMajorAxis") (declared-name "semiMajorAxis") (range (start (line 75) (character 1)) (end (line 75) (character 66))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::semiMinorAxis"))) (kind "attribute def") (name "semiMinorAxis") (declared-name "semiMinorAxis") (range (start (line 76) (character 1)) (end (line 76) (character 66))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 17) (character 1)) (end (line 17) (character 40))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 39))))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::xoffset"))) (kind "attribute def") (name "xoffset") (declared-name "xoffset") (range (start (line 77) (character 1)) (end (line 77) (character 74))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ShapeItems::yoffset"))) (kind "attribute def") (name "yoffset") (declared-name "yoffset") (range (start (line 78) (character 1)) (end (line 78) (character 74))) (parent (node (document "d0") (qualified-name "ShapeItems"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQSpaceTime::*") (range (start (line 8) (character 16)) (end (line 8) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 9) (character 16)) (end (line 9) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Objects::*") (range (start (line 12) (character 16)) (end (line 12) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 6) (character 16)) (end (line 6) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Circle"))) (kind specialization) (ordinal 0)) (authored-target "Ellipse") (range (start (line 106) (character 20)) (end (line 106) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Circle::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 112) (character 16)) (end (line 112) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Circle::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (range (start (line 113) (character 16)) (end (line 113) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Circle::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (range (start (line 114) (character 16)) (end (line 114) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Circle::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCone"))) (kind specialization) (ordinal 0)) (authored-target "Cone") (range (start (line 463) (character 26)) (end (line 463) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cone")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 469) (character 16)) (end (line 469) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (range (start (line 470) (character 16)) (end (line 470) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (range (start (line 471) (character 16)) (end (line 471) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder"))) (kind specialization) (ordinal 0)) (authored-target "Cylinder") (range (start (line 512) (character 30)) (end (line 512) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 518) (character 16)) (end (line 518) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (range (start (line 519) (character 16)) (end (line 519) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (range (start (line 520) (character 16)) (end (line 520) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc"))) (kind specialization) (ordinal 0)) (authored-target "Disc") (range (start (line 260) (character 26)) (end (line 260) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Disc")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 266) (character 16)) (end (line 266) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (range (start (line 267) (character 16)) (end (line 267) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (range (start (line 268) (character 16)) (end (line 268) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularDisc::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Cone"))) (kind specialization) (ordinal 0)) (authored-target "ConeOrCylinder") (range (start (line 440) (character 18)) (end (line 440) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (range (start (line 383) (character 28)) (end (line 383) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus"))) (kind redefinition) (ordinal 0)) (authored-target "genus") (range (start (line 437) (character 16)) (end (line 437) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::genus")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (range (start (line 393) (character 16)) (end (line 393) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::height")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (range (start (line 391) (character 16)) (end (line 391) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (range (start (line 392) (character 16)) (end (line 392) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (range (start (line 395) (character 16)) (end (line 395) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset"))) (kind redefinition) (ordinal 0)) (authored-target "yoffset") (range (start (line 396) (character 16)) (end (line 396) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder::yoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 0)) (authored-target "Path") (range (start (line 82) (character 26)) (end (line 82) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Path")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConicSection"))) (kind specialization) (ordinal 1)) (authored-target "PlanarCurve") (range (start (line 82) (character 32)) (end (line 82) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConicSurface"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (range (start (line 277) (character 26)) (end (line 277) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus"))) (kind redefinition) (ordinal 0)) (authored-target "genus") (range (start (line 287) (character 16)) (end (line 287) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface::genus")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Cuboid"))) (kind specialization) (ordinal 0)) (authored-target "CuboidOrTriangularPrism") (range (start (line 745) (character 20)) (end (line 745) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism"))) (kind specialization) (ordinal 0)) (authored-target "Polyhedron") (range (start (line 561) (character 37)) (end (line 561) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Cylinder"))) (kind specialization) (ordinal 0)) (authored-target "ConeOrCylinder") (range (start (line 488) (character 22)) (end (line 488) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConeOrCylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (range (start (line 236) (character 18)) (end (line 236) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Disc"))) (kind specialization) (ordinal 1)) (authored-target "PlanarSurface") (range (start (line 236) (character 25)) (end (line 236) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (range (start (line 242) (character 16)) (end (line 242) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Disc::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (range (start (line 243) (character 16)) (end (line 243) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Disc::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::EccentricCone"))) (kind specialization) (ordinal 0)) (authored-target "Cone") (range (start (line 454) (character 27)) (end (line 454) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cone")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::EccentricCylinder"))) (kind specialization) (ordinal 0)) (authored-target "Cylinder") (range (start (line 503) (character 31)) (end (line 503) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipse"))) (kind specialization) (ordinal 0)) (authored-target "ConicSection") (range (start (line 94) (character 21)) (end (line 94) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMajorAxis") (range (start (line 100) (character 16)) (end (line 100) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMajorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis"))) (kind redefinition) (ordinal 0)) (authored-target "semiMinorAxis") (range (start (line 101) (character 16)) (end (line 101) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Ellipse::semiMinorAxis")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid"))) (kind specialization) (ordinal 0)) (authored-target "ConicSurface") (range (start (line 290) (character 23)) (end (line 290) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis1"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 296) (character 43)) (end (line 296) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis2"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 297) (character 43)) (end (line 297) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Ellipsoid::semiAxis3"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 298) (character 43)) (end (line 298) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola"))) (kind specialization) (ordinal 0)) (authored-target "ConicSection") (range (start (line 132) (character 23)) (end (line 132) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::conjugateAxis"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 139) (character 47)) (end (line 139) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperbola::tranverseAxis"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 138) (character 47)) (end (line 138) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid"))) (kind specialization) (ordinal 0)) (authored-target "ConicSurface") (range (start (line 326) (character 25)) (end (line 326) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::conjugateAxis"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 333) (character 47)) (end (line 333) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Hyperboloid::transverseAxis"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 332) (character 48)) (end (line 332) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Item"))) (kind membershipImport) (ordinal 0)) (authored-target "Items::Item") (range (start (line 13) (character 16)) (end (line 13) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Line"))) (kind specialization) (ordinal 0)) (authored-target "PlanarCurve") (range (start (line 48) (character 18)) (end (line 48) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Line::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (range (start (line 54) (character 16)) (end (line 54) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Line::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension"))) (kind redefinition) (ordinal 0)) (authored-target "outerSpaceDimension") (range (start (line 55) (character 16)) (end (line 55) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Line::outerSpaceDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::MatesWith"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::MatesWith") (range (start (line 11) (character 16)) (end (line 11) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Parabola"))) (kind specialization) (ordinal 0)) (authored-target "ConicSection") (range (start (line 121) (character 22)) (end (line 121) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSection")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Parabola::focalDistance"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 127) (character 47)) (end (line 127) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid"))) (kind specialization) (ordinal 0)) (authored-target "ConicSurface") (range (start (line 315) (character 24)) (end (line 315) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::ConicSurface")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Paraboloid::focalDistance"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 321) (character 47)) (end (line 321) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Path"))) (kind specialization) (ordinal 0)) (authored-target "StructuredSpaceObject::StructuredCurve") (range (start (line 58) (character 27)) (end (line 58) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve"))) (kind specialization) (ordinal 0)) (authored-target "Curve") (range (start (line 24) (character 25)) (end (line 24) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (range (start (line 30) (character 16)) (end (line 30) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension"))) (kind redefinition) (ordinal 0)) (authored-target "outerSpaceDimension") (range (start (line 32) (character 16)) (end (line 32) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve::outerSpaceDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface"))) (kind specialization) (ordinal 0)) (authored-target "Surface") (range (start (line 36) (character 27)) (end (line 36) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area"))) (kind redefinition) (ordinal 0)) (authored-target "area") (range (start (line 42) (character 16)) (end (line 42) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::area")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension"))) (kind redefinition) (ordinal 0)) (authored-target "outerSpaceDimension") (range (start (line 43) (character 16)) (end (line 43) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarSurface::outerSpaceDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 0)) (authored-target "Path") (range (start (line 142) (character 21)) (end (line 142) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Path")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polygon"))) (kind specialization) (ordinal 1)) (authored-target "PlanarCurve") (range (start (line 142) (character 27)) (end (line 142) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::PlanarCurve")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed"))) (kind redefinition) (ordinal 0)) (authored-target "isClosed") (range (start (line 150) (character 16)) (end (line 150) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polygon::isClosed")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (range (start (line 540) (character 24)) (end (line 540) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus"))) (kind redefinition) (ordinal 0)) (authored-target "genus") (range (start (line 558) (character 16)) (end (line 558) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::genus")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed"))) (kind redefinition) (ordinal 0)) (authored-target "isClosed") (range (start (line 546) (character 16)) (end (line 546) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::isClosed")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension"))) (kind redefinition) (ordinal 0)) (authored-target "outerSpaceDimension") (range (start (line 556) (character 16)) (end (line 556) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron::outerSpaceDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (range (start (line 7) (character 16)) (end (line 7) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid"))) (kind specialization) (ordinal 0)) (authored-target "Polyhedron") (range (start (line 822) (character 21)) (end (line 822) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polyhedron")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (range (start (line 830) (character 16)) (end (line 830) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::height")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::wallNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "Positive") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Positive")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (range (start (line 831) (character 16)) (end (line 831) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset"))) (kind redefinition) (ordinal 0)) (authored-target "yoffset") (range (start (line 832) (character 16)) (end (line 832) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid::yoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Quadrilateral"))) (kind specialization) (ordinal 0)) (authored-target "Polygon") (range (start (line 195) (character 27)) (end (line 195) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polygon")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Rectangle"))) (kind specialization) (ordinal 0)) (authored-target "Quadrilateral") (range (start (line 214) (character 23)) (end (line 214) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Quadrilateral")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Rectangle::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (range (start (line 220) (character 16)) (end (line 220) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Rectangle::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Rectangle::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (range (start (line 221) (character 16)) (end (line 221) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Rectangle::width")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid"))) (kind specialization) (ordinal 0)) (authored-target "Cuboid") (range (start (line 797) (character 31)) (end (line 797) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Cuboid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (range (start (line 805) (character 16)) (end (line 805) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::height")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (range (start (line 803) (character 16)) (end (line 803) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (range (start (line 804) (character 16)) (end (line 804) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularCuboid::width")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid"))) (kind specialization) (ordinal 0)) (authored-target "Pyramid") (range (start (line 882) (character 32)) (end (line 882) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength"))) (kind redefinition) (ordinal 0)) (authored-target "baseLength") (range (start (line 888) (character 16)) (end (line 888) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseLength")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth"))) (kind redefinition) (ordinal 0)) (authored-target "baseWidth") (range (start (line 889) (character 16)) (end (line 889) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RectangularPyramid::baseWidth")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid"))) (kind specialization) (ordinal 0)) (authored-target "Toroid") (range (start (line 367) (character 31)) (end (line 367) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Toroid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleLength"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 373) (character 49)) (end (line 373) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RectangularToroid::rectangleWidth"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 374) (character 49)) (end (line 374) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone"))) (kind specialization) (ordinal 0)) (authored-target "CircularCone") (range (start (line 478) (character 31)) (end (line 478) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCone")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (range (start (line 484) (character 16)) (end (line 484) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset"))) (kind redefinition) (ordinal 0)) (authored-target "yoffset") (range (start (line 485) (character 16)) (end (line 485) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCone::yoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder"))) (kind specialization) (ordinal 0)) (authored-target "CircularCylinder") (range (start (line 530) (character 35)) (end (line 530) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CircularCylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (range (start (line 536) (character 16)) (end (line 536) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset"))) (kind redefinition) (ordinal 0)) (authored-target "yoffset") (range (start (line 537) (character 16)) (end (line 537) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightCircularCylinder::yoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangle"))) (kind specialization) (ordinal 0)) (authored-target "Triangle") (range (start (line 180) (character 27)) (end (line 180) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Triangle")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (range (start (line 186) (character 16)) (end (line 186) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangle::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism"))) (kind specialization) (ordinal 0)) (authored-target "TriangularPrism") (range (start (line 709) (character 34)) (end (line 709) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::TriangularPrism")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (range (start (line 718) (character 16)) (end (line 718) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::height")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (range (start (line 716) (character 16)) (end (line 716) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (range (start (line 717) (character 16)) (end (line 717) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::RightTriangularPrism::width")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Shell"))) (kind specialization) (ordinal 0)) (authored-target "StructuredSpaceObject::StructuredSurface") (range (start (line 229) (character 28)) (end (line 229) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere"))) (kind specialization) (ordinal 0)) (authored-target "Ellipsoid") (range (start (line 303) (character 20)) (end (line 303) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Ellipsoid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 309) (character 16)) (end (line 309) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1"))) (kind redefinition) (ordinal 0)) (authored-target "semiAxis1") (range (start (line 310) (character 16)) (end (line 310) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis1")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2"))) (kind redefinition) (ordinal 0)) (authored-target "semiAxis2") (range (start (line 311) (character 16)) (end (line 311) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis2")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3"))) (kind redefinition) (ordinal 0)) (authored-target "semiAxis3") (range (start (line 312) (character 16)) (end (line 312) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Sphere::semiAxis3")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron"))) (kind specialization) (ordinal 0)) (authored-target "Pyramid") (range (start (line 865) (character 25)) (end (line 865) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Pyramid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength"))) (kind redefinition) (ordinal 0)) (authored-target "baseLength") (range (start (line 871) (character 16)) (end (line 871) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseLength")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth"))) (kind redefinition) (ordinal 0)) (authored-target "baseWidth") (range (start (line 872) (character 16)) (end (line 872) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Tetrahedron::baseWidth")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Toroid"))) (kind specialization) (ordinal 0)) (authored-target "Shell") (range (start (line 336) (character 20)) (end (line 336) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Shell")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Toroid::genus"))) (kind redefinition) (ordinal 0)) (authored-target "genus") (range (start (line 351) (character 16)) (end (line 351) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Toroid::genus")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 343) (character 50)) (end (line 343) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Torus"))) (kind specialization) (ordinal 0)) (authored-target "Toroid") (range (start (line 354) (character 19)) (end (line 354) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Toroid")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Torus::majorRadius"))) (kind redefinition) (ordinal 0)) (authored-target "revolutionRadius") (range (start (line 360) (character 28)) (end (line 360) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Toroid::revolutionRadius")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Torus::minorRadius"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 361) (character 45)) (end (line 361) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Triangle"))) (kind specialization) (ordinal 0)) (authored-target "Polygon") (range (start (line 158) (character 22)) (end (line 158) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Polygon")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Triangle::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (range (start (line 165) (character 16)) (end (line 165) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::length")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Triangle::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (range (start (line 166) (character 16)) (end (line 166) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::width")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset"))) (kind redefinition) (ordinal 0)) (authored-target "xoffset") (range (start (line 167) (character 16)) (end (line 167) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::Triangle::xoffset")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::TriangularPrism"))) (kind specialization) (ordinal 0)) (authored-target "CuboidOrTriangularPrism") (range (start (line 679) (character 29)) (end (line 679) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ShapeItems::CuboidOrTriangularPrism")))))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::baseLength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::baseWidth"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::equals"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::equals") (range (start (line 14) (character 16)) (end (line 14) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::exists"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::exists") (range (start (line 21) (character 16)) (end (line 21) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 20) (character 16)) (end (line 20) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::if"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::if") (range (start (line 19) (character 16)) (end (line 19) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (range (start (line 18) (character 16)) (end (line 18) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (range (start (line 15) (character 16)) (end (line 15) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::m"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::m") (range (start (line 10) (character 16)) (end (line 10) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (range (start (line 16) (character 16)) (end (line 16) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::scalarQuantities"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::scalarQuantities") (range (start (line 22) (character 16)) (end (line 22) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::semiMajorAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::semiMinorAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 17) (character 16)) (end (line 17) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::xoffset"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ShapeItems::yoffset"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
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
