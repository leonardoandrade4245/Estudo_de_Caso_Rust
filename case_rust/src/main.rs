use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    marca: String,
    categoria: String,
}

struct SistemaBusca {
    produtos: Vec<Produto>,
    indice_nome: HashMap<String, Vec<u32>>,
    indice_marca: HashMap<String, Vec<u32>>,
    indice_categoria: HashMap<String, Vec<u32>>,
    proximo_id: u32,
}

impl SistemaBusca {
    fn new() -> Self {
        SistemaBusca {
            produtos: Vec::new(),
            indice_nome: HashMap::new(),
            indice_marca: HashMap::new(),
            indice_categoria: HashMap::new(),
            proximo_id: 1,
        }
    }

    fn adicionar_produto(&mut self, nome: String, marca: String, categoria: String) {
        let produto = Produto {
            id: self.proximo_id,
            nome: nome.clone(),
            marca: marca.clone(),
            categoria: categoria.clone(),
        };

        self.proximo_id += 1;
        let id = produto.id;

        self.produtos.push(produto);

        self.indice_nome
            .entry(nome.to_lowercase())
            .or_default()
            .push(id);
        self.indice_marca
            .entry(marca.to_lowercase())
            .or_default()
            .push(id);
        self.indice_categoria
            .entry(categoria.to_lowercase())
            .or_default()
            .push(id);
    }

    // Popula com 5 produtos por categoria
    fn popular_inicial(&mut self) {
        let categorias = vec!["Eletrônicos", "Celulares", "Periféricos"];

        for categoria in categorias {
            for i in 1..=5 {
                let nome = format!("{} Produto {}", categoria, i);
                let marca = format!("Marca{}", i);
                self.adicionar_produto(nome, marca, categoria.to_string());
            }
        }
    }

    fn buscar(&self, termo: &str, opcao: u32) -> Vec<&Produto> {
        let termo = termo.to_lowercase();
        let mut ids_encontrados = Vec::new();

        match opcao {
            1 => {
                if let Some(ids) = self.indice_nome.get(&termo) {
                    ids_encontrados.extend(ids);
                }
            }
            2 => {
                if let Some(ids) = self.indice_marca.get(&termo) {
                    ids_encontrados.extend(ids);
                }
            }
            3 => {
                if let Some(ids) = self.indice_categoria.get(&termo) {
                    ids_encontrados.extend(ids);
                }
            }
            _ => {
                println!("Opção inválida.");
                return Vec::new();
            }
        }

        ids_encontrados.sort_unstable();
        ids_encontrados.dedup();

        ids_encontrados
            .iter()
            .filter_map(|&id| self.produtos.iter().find(|p| p.id == id))
            .collect()
    }

    fn listar_estoque(&self) {
        println!("\n==== Estoque Atual ====");
        if self.produtos.is_empty() {
            println!("Estoque vazio.");
        } else {
            for produto in &self.produtos {
                println!(
                    "ID: {}, Nome: {}, Marca: {}, Categoria: {}",
                    produto.id, produto.nome, produto.marca, produto.categoria
                );
            }
        }
        println!("========================\n");
    }

    fn listar_por_categoria(&self, categoria: &str) {
        let categoria_lower = categoria.to_lowercase();
        let produtos_categoria: Vec<&Produto> = self
            .produtos
            .iter()
            .filter(|p| p.categoria.to_lowercase() == categoria_lower)
            .collect();

        println!("\n=== Produtos na categoria '{}' ===", categoria);
        if produtos_categoria.is_empty() {
            println!("Nenhum produto encontrado nessa categoria.");
        } else {
            for produto in produtos_categoria {
                println!(
                    "ID: {}, Nome: {}, Marca: {}, Categoria: {}",
                    produto.id, produto.nome, produto.marca, produto.categoria
                );
            }
        }
        println!("==================================\n");
    }
}

fn ler_input(mensagem: &str) -> String {
    let mut entrada = String::new();
    println!("{}", mensagem);
    io::stdin().read_line(&mut entrada).expect("Erro na leitura");
    entrada.trim().to_string()
}

fn main() {
    let mut sistema = SistemaBusca::new();

    // Popula com 5 produtos por categoria
    sistema.popular_inicial();

    // Variável para guardar a categoria da última pesquisa
    let mut ultima_categoria_pesquisada: Option<String> = None;

    // Mostra o estoque completo antes do menu
    sistema.listar_estoque();

    loop {
        // Se já houve pesquisa, mostra produtos da categoria da última busca
        if let Some(categoria) = &ultima_categoria_pesquisada {
            sistema.listar_por_categoria(categoria);
        } else {
            // Se for a primeira vez (nenhuma pesquisa feita), só mostra o menu
            println!("(Estoque completo já exibido no início)");
        }

        println!("========== Menu ==========");
        println!("1 - Ver Estoque");
        println!("2 - Adicionar Produto");
        println!("3 - Pesquisar Produto");
        println!("4 - Sair");
        println!("==========================");

        let opcao = ler_input("Escolha uma opção:");
        match opcao.as_str() {
            "1" => {
                sistema.listar_estoque();
            }
            "2" => {
                let nome = ler_input("Digite o nome do produto:");
                let marca = ler_input("Digite a marca do produto:");
                let categoria = ler_input("Digite a categoria do produto:");

                sistema.adicionar_produto(nome, marca, categoria);
                println!("Produto adicionado com sucesso!\n");
            }
            "3" => {
                println!("Escolha o tipo de busca:");
                println!("1 - Buscar por Nome");
                println!("2 - Buscar por Marca");
                println!("3 - Buscar por Categoria");

                let opcao_busca = ler_input("Digite o número da opção:");
                let opcao_busca: u32 = opcao_busca.trim().parse().unwrap_or(0);

                let termo = ler_input("Digite o termo de busca:");

                let resultados = sistema.buscar(&termo, opcao_busca);

                println!("\n=== Resultados da busca ===");
                if resultados.is_empty() {
                    println!("Nenhum produto encontrado.");
                    // Não altera categoria da última pesquisa
                } else {
                    for produto in &resultados {
                        println!(
                            "ID: {}, Nome: {}, Marca: {}, Categoria: {}",
                            produto.id, produto.nome, produto.marca, produto.categoria
                        );
                    }
                    // Atualiza a categoria da última pesquisa com a do primeiro resultado encontrado
                    ultima_categoria_pesquisada = Some(resultados[0].categoria.clone());
                }
                println!("============================\n");
            }
            "4" => {
                println!("Encerrando o programa.");
                break;
            }
            _ => {
                println!("Opção inválida. Tente novamente.\n");
            }
        }
    }
}
