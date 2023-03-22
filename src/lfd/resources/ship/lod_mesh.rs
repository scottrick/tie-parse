use core::fmt::Debug;
use core::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use byteorder::ReadBytesExt;

use crate::lfd::traits::lfd_print::LfdPrint;

use super::shape::Shape;
use super::shape_settings::ShapeSettings;
use super::vertex16::Vertex16;
use super::vertex_array::VertexArray;

// LodMesh
/// {
///   /* 0x00 */ byte Signature
///   /* 0x01 */ byte Unknown
///   /* 0x02 */ byte NumVertices
///   /* 0x03 */ byte Unknown
///   /* 0x04 */ byte NumShapes
///   /* 0x05 */ byte[NumShapes] ColorIndices
///              Vertex16 MinimumBound
///              Vertex16 MaximumBound
///              Vertex16[NumVertices] MeshVertices
///              Vertex16[NumVertices] VertexNormals
///              ShapeSettings[NumShapes]
///              Shape[NumShapes]    MeshGeometry
///              Unknown1[NumShapes]
///              short NumUnk2
///              Unknown2[NumUnk2]
///              Unknown3[NumUnk2]
/// }
pub struct LodMesh {
    _signature: u8,
    _unknown_1: u8,
    num_vertices: u8,
    _unknown_2: u8,
    num_shapes: u8,
    color_indices: Vec<u8>,
    min_bound: Vertex16,
    max_bound: Vertex16,
    mesh_vertices: VertexArray,
    vertex_normals: VertexArray,
    shape_settings: ShapeSettings,
    shapes: Vec<Shape>,
}

impl LodMesh {
    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, String> {
        let _signature: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading signature: {e}"))?;

        let _unknown_1: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading unknown_1: {e}"))?;

        let num_vertices: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading num_vertices: {e}"))?;

        let _unknown_2: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading unknown_2: {e}"))?;

        let num_shapes: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading num_shapes: {e}"))?;

        let mut color_indices: Vec<u8> = vec![0; num_shapes as usize];

        reader
            .read_exact(&mut color_indices)
            .map_err(|e| format!("Error reading color_indices: {e}"))?;

        let min_bound =
            Vertex16::from_reader(reader).map_err(|e| format!("Error reading Vertex16: {e}"))?;

        let max_bound =
            Vertex16::from_reader(reader).map_err(|e| format!("Error reading Vertex16: {e}"))?;

        let mesh_vertices = VertexArray::from_reader(reader, num_vertices, true)
            .map_err(|e| format!("Error reading MeshVertices: {e}"))?;

        let vertex_normals = VertexArray::from_reader(reader, num_vertices, false)
            .map_err(|e| format!("Error reading VertexNormals: {e}"))?;

        let shape_settings = ShapeSettings::from_reader(reader)
            .map_err(|e| format!("Error reading ShapeSettings: {e}"))?;

        let mut shapes: Vec<Shape> = Vec::new();

        for _ in 0..num_shapes {
            let new_shape =
                Shape::from_reader(reader).map_err(|e| format!("Error reading Shape: {e}"))?;

            shapes.push(new_shape);
        }

        Ok(Self {
            _signature,
            _unknown_1,
            num_vertices,
            _unknown_2,
            num_shapes,
            color_indices,
            min_bound,
            max_bound,
            mesh_vertices,
            vertex_normals,
            shape_settings,
            shapes,
        })
    }

    pub fn obj_to_writer(&self, writer: &mut dyn std::io::Write) -> Result<(), String> {
        for v in self.mesh_vertices.vertices.iter() {
            writeln!(writer, "v {:?} {:?} {:?}", v.x, v.y, v.z)
                .map_err(|e| format!("Error writing mesh vertices to obj writer: {e}"))?;
        }

        Ok(())
    }
}

impl Debug for LodMesh {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = "LodMesh";
        f.write_str(debug_string)
    }
}

impl LfdPrint for LodMesh {
    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
        println!("{spaces} num_vertices: {:?}", self.num_vertices);
        println!("{spaces} num_shapes: {:?}", self.num_shapes);
        println!("{spaces} color_indices: {:?}", self.color_indices);
        println!("{spaces} min_bound: {:?}", self.min_bound);
        println!("{spaces} max_bound: {:?}", self.max_bound);
        println!(
            "{spaces} MeshVertices[{:?}]",
            self.mesh_vertices.vertices.len()
        );
        println!(
            "{spaces} VertexNormals[{:?}]",
            self.vertex_normals.vertices.len()
        );
        println!("{spaces} ShapeSettings[{:?}]", self.shape_settings);
        for shape in self.shapes.iter() {
            println!("{spaces}  {:?}", shape);
        }
    }

    fn lfd_get_print_str(&self) -> String {
        format!("{:?}", self)
    }
}
