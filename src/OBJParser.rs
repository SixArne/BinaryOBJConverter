use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::mem::size_of;

struct Normal
{
    x: f32,
    y: f32,
    z: f32,
}

struct Face
{
    v1: i32,
    v2: i32,
    v3: i32,
}

struct Vertex
{
    x: f32,
    y: f32,
    z: f32,
}

pub struct OBJ
{
    vertex_count: i32,
    vertices: Vec<Vertex>,

    normal_count: i32,
    normals: Vec<Normal>,

    face_count: i32,
    faces: Vec<Face>,
}

impl OBJ
{
    pub fn new() -> OBJ
    {
        OBJ
        {
            vertex_count: 0,
            vertices: Vec::new(),
            normal_count: 0,
            normals: Vec::new(),
            face_count: 0,
            faces: Vec::new(),
        }
    }

    fn reset(&mut self)
    {
        self.vertex_count = 0;
        self.vertices.clear();
        self.normal_count = 0;
        self.normals.clear();
        self.face_count = 0;
        self.faces.clear();
    }

    pub fn ReadFile(&mut self, path: &String)
    {
        let file = File::open(path).expect("File not found");
        let reader = BufReader::new(file);

        for line in reader.lines()
        {
            match line
            {
                Ok(text) => self.read_obj_line(&text),
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    pub fn ReadBinary(&mut self, path: &String)
    {
        self.reset();

        let file = File::open(path).expect("file not found");
        let mut reader = BufReader::new(file);

        // read vertex count
        let mut vertex_count_buf = [0u8; size_of::<i32>()];
        reader.read_exact(&mut vertex_count_buf).unwrap();
        self.vertex_count = i32::from_le_bytes(vertex_count_buf);

        // read in vertices
        for _ in 0..self.vertex_count
        {
            let mut vertex_buf = [0u8; size_of::<f32>() * 3];
            reader.read_exact(&mut vertex_buf).unwrap();
            let vertex = Vertex
            {
                x: f32::from_le_bytes([vertex_buf[0], vertex_buf[1], vertex_buf[2],vertex_buf[3]]),
                y: f32::from_le_bytes([vertex_buf[4], vertex_buf[5], vertex_buf[6],vertex_buf[7]]),
                z: f32::from_le_bytes([vertex_buf[8], vertex_buf[9], vertex_buf[10],vertex_buf[11]]),
            };
            self.vertices.push(vertex);
        }

        // read normal count
        let mut normal_count_buf = [0u8; size_of::<i32>()];
        reader.read_exact(&mut normal_count_buf).unwrap();
        self.normal_count = i32::from_le_bytes(normal_count_buf);

        // read in normals
        for _ in 0..self.normal_count
        {
            let mut normal_buf = [0u8; size_of::<f32>() * 3];
            reader.read_exact(&mut normal_buf).unwrap();
            let normal = Normal
            {
                x: f32::from_le_bytes([normal_buf[0], normal_buf[1], normal_buf[2],normal_buf[3]]),
                y: f32::from_le_bytes([normal_buf[4], normal_buf[5], normal_buf[6],normal_buf[7]]),
                z: f32::from_le_bytes([normal_buf[8], normal_buf[9], normal_buf[10],normal_buf[11]]),
            };
            self.normals.push(normal);
        }

        // read face count
        let mut face_count_buf = [0u8; size_of::<i32>()];
        reader.read_exact(&mut face_count_buf).unwrap();
        self.face_count = i32::from_le_bytes(face_count_buf);

        // read in faces
        for _ in 0..self.face_count
        {
            let mut face_buf = [0u8; size_of::<i32>() * 3];
            reader.read_exact(&mut face_buf).unwrap();
            let face = Face
            {
                v1: i32::from_le_bytes([face_buf[0], face_buf[1], face_buf[2],face_buf[3]]),
                v2: i32::from_le_bytes([face_buf[4], face_buf[5], face_buf[6],face_buf[7]]),
                v3: i32::from_le_bytes([face_buf[8], face_buf[9], face_buf[10],face_buf[11]]),
            };
            self.faces.push(face);
        }
    }

    pub fn WriteAsBinary(&mut self, path: &String)
    {
        let mut file = File::create(path).unwrap();

        // Write vertexCount
        file.write_all(&self.vertex_count.to_le_bytes());
        for vertex in &self.vertices
        {
            file.write_all(&vertex.x.to_le_bytes());
            file.write_all(&vertex.y.to_le_bytes());
            file.write_all(&vertex.z.to_le_bytes());
        }

        file.write_all(&self.normal_count.to_le_bytes());
        for normal in &self.normals
        {
            file.write_all(&normal.x.to_le_bytes());
            file.write_all(&normal.y.to_le_bytes());
            file.write_all(&normal.z.to_le_bytes());
        }

        file.write_all(&self.face_count.to_le_bytes());
        for face in &self.faces
        {
            file.write_all(&face.v1.to_le_bytes());
            file.write_all(&face.v2.to_le_bytes());
            file.write_all(&face.v3.to_le_bytes());
        }
    }

    pub fn WriteAsText(&mut self, path: &String)
    {
        let mut file = File::create(path).unwrap();

        for vertex in &self.vertices
        {
            write!(file, "v {} {} {}\n", vertex.x, vertex.y, vertex.z).expect("Failed to write vertex to file");
        }

        for normal in &self.normals
        {
            write!(file, "vn {} {} {}\n", normal.x, normal.y, normal.z).expect("Failed to write normal to file");
        }

        for face in &self.faces
        {
            write!(file, "f {} {} {}\n", face.v1, face.v2, face.v3).expect("Failed to write face to file");
        }
    }

    fn read_obj_line(&mut self, line: &String)
    {
        if line.starts_with("v ")
        {
            let vertex: Vertex = extract_vertex_from_line(line);
            self.vertices.push(vertex);
            self.vertex_count += 1;
        }
        else if line.starts_with("f ")
        {
            let face: Face = extract_face_from_line(line);
            self.faces.push(face);
            self.face_count += 1;
        }
        else if line.starts_with("vn ")
        {
            let normal: Normal = extract_normal_from_line(line);
            self.normals.push(normal);
            self.normal_count += 1;
        }
    }
}

fn extract_vertex_from_line(line: &String) -> Vertex
{
    let values: Vec<&str> = line.split_whitespace().collect();
    let x = values[1].parse::<f32>().unwrap();
    let y = values[2].parse::<f32>().unwrap();
    let z = values[3].parse::<f32>().unwrap();

    return Vertex{x, y, z};
}

fn extract_face_from_line(line: &String) -> Face
{
    let values: Vec<&str> = line.split_whitespace().collect();
    let v1 = values[1].parse::<i32>().unwrap();
    let v2 = values[2].parse::<i32>().unwrap();
    let v3 = values[3].parse::<i32>().unwrap();

    return Face{v1, v2, v3};
}

fn extract_normal_from_line(line: &String) -> Normal
{
    let values: Vec<&str> = line.split_whitespace().collect();
    let x = values[1].parse::<f32>().unwrap();
    let y = values[2].parse::<f32>().unwrap();
    let z = values[3].parse::<f32>().unwrap();

    return Normal{x, y, z};
}