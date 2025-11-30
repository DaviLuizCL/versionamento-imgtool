use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
use image::{DynamicImage, ImageFormat};
use serde::Serialize;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "img-tool",
    version,
    about = "Ferramenta de linha de comando para processar imagens (conversão, resize, grayscale e relatório)"
)]
struct Cli {
    /// Arquivo ou diretório de entrada
    input: PathBuf,

    /// Diretório de saída
    #[arg(long, default_value = "output")]
    output: PathBuf,

    /// Formato de saída (jpg ou png por enquanto)
    #[arg(long)]
    to_format: Option<String>,

    /// Redimensionar para LARGURAxALTURA (ex: 800x600)
    #[arg(long)]
    resize: Option<String>,

    /// Converter para tons de cinza
    #[arg(long)]
    grayscale: bool,

    /// Caminho para salvar relatório em JSON
    #[arg(long)]
    report: Option<PathBuf>,
}

#[derive(Serialize, Debug)]
struct ImageReport {
    input: String,
    output: String,
    original_format: String,
    new_format: String,
    original_size: u64,
    new_size: u64,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Garante que o diretório de saída existe
    fs::create_dir_all(&args.output)?;

    let mut reports = Vec::new();

    let paths = collect_paths(&args.input)?;

    if paths.is_empty() {
        eprintln!("Nenhum arquivo encontrado para processar.");
        return Ok(());
    }

    println!("Encontrados {} arquivo(s) para processar.", paths.len());

    for path in paths {
        match process_image(&path, &args) {
            Ok(Some(report)) => {
                println!("OK  -> {}", report.output);
                reports.push(report);
            }
            Ok(None) => {
                println!("IGN -> {}", path.display());
            }
            Err(e) => {
                eprintln!("ERR -> {}: {e}", path.display());
            }
        }
    }

    // Se foi pedido relatório, salva em JSON
    if let Some(report_path) = args.report {
        let json = serde_json::to_string_pretty(&reports)?;
        fs::write(&report_path, json)?;
        println!("Relatório salvo em: {}", report_path.display());
    }

    Ok(())
}

/// Coleta todos os caminhos de arquivos a partir de um arquivo único ou diretório.
fn collect_paths(input: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if input.is_file() {
        files.push(input.to_path_buf());
    } else if input.is_dir() {
        for entry in WalkDir::new(input) {
            let entry = entry?;
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    } else {
        eprintln!("Entrada não é arquivo nem diretório: {}", input.display());
    }

    Ok(files)
}

/// Processa uma única imagem: aplica resize, grayscale, conversão de formato
/// e gera um registro para o relatório.
fn process_image(path: &Path, args: &Cli) -> Result<Option<ImageReport>> {
    // Lê metadados
    let metadata = fs::metadata(path)?;
    let original_size = metadata.len();

    // Lê binário do arquivo
    let data = fs::read(path)?;

    // Tenta identificar o formato
    let format = match image::guess_format(&data) {
        Ok(f) => f,
        Err(_) => {
            // Se não for imagem, ignora
            return Ok(None);
        }
    };

    let original_format = format!("{:?}", format);

    // Carrega imagem
    let mut img = image::load_from_memory(&data)?;

    // Aplica resize se solicitado
    if let Some(resize_str) = &args.resize {
        if let Some((w, h)) = parse_resize(resize_str) {
            img = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);
        } else {
            eprintln!("Parâmetro inválido em --resize (use LARGURAxALTURA), ignorando resize.");
        }
    }

    // Aplica grayscale se solicitado
    if args.grayscale {
        img = DynamicImage::ImageLuma8(img.to_luma8());
    }

    // Define formato de saída
    let new_format = args
        .to_format
        .as_deref()
        .unwrap_or_else(|| default_output_format(format));

    let output_path = build_output_path(path, &args.output, new_format);

    // Codifica e salva imagem de saída
    let mut out_buf: Vec<u8> = Vec::new();

    match new_format {
        "jpg" | "jpeg" => {
            img.write_to(&mut out_buf, ImageFormat::Jpeg)?;
        }
        "png" => {
            img.write_to(&mut out_buf, ImageFormat::Png)?;
        }
        other => {
            eprintln!(
                "Formato de saída não suportado ({other}), usando PNG como fallback."
            );
            img.write_to(&mut out_buf, ImageFormat::Png)?;
        }
    }

    fs::write(&output_path, &out_buf)?;

    let new_size = fs::metadata(&output_path)?.len();

    Ok(Some(ImageReport {
        input: path.display().to_string(),
        output: output_path.display().to_string(),
        original_format,
        new_format: new_format.to_string(),
        original_size,
        new_size,
    }))
}

/// Interpreta uma string do tipo "800x600" como (800, 600)
fn parse_resize(s: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return None;
    }
    let w = parts[0].parse().ok()?;
    let h = parts[1].parse().ok()?;
    Some((w, h))
}

/// Define um formato padrão de saída se o usuário não especificar.
fn default_output_format(input_format: ImageFormat) -> &'static str {
    match input_format {
        ImageFormat::Png => "jpg",
        ImageFormat::Jpeg => "png",
        _ => "png",
    }
}

/// Monta o caminho de saída baseado no diretório de saída e na nova extensão.
fn build_output_path(input: &Path, output_dir: &Path, new_ext: &str) -> PathBuf {
    let file_stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let mut out = output_dir.to_path_buf();
    out.push(format!("{file_stem}.{new_ext}"));
    out
}
