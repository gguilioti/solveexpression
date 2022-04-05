use std::io;
use regex::Regex;

struct Parenteses
{
    pos_inicial: i32,
    pos_final: i32,
    nivel: i32,
}

fn main() {
    let mut expressao = String::new();

    io::stdin().read_line(&mut expressao);

    tokenize(&expressao.to_owned());
}

fn tokenize(expressao: &str) // divide a expressão em tokens, utiliza um vetor de Strings
{
    let mut expressao_auxiliar: Vec<String> = Vec::<String>::new();
    let re = Regex::new(r"\s?(-?\d+|[+*\-/()])\s?").unwrap();

    for cap in re.captures_iter(expressao) {
        expressao_auxiliar.push(String::from(&cap[1]));
    }
    
    add_nivel_to_parenteses(&mut expressao_auxiliar);
}

fn add_nivel_to_parenteses(expressao: &mut Vec<String>) //Adiciona nivel aos parenteses
{
    let mut nivel: i32 = 0;
    let mut vetor_parenteses = vec![];
    
    for i in 0..expressao.len() //Percorre toda a expressão
    {
        if expressao[i].to_string() == "(" //Encontra um abre parenteses
        {
            nivel = nivel+1; //Aumenta o nível
            vetor_parenteses.push(Parenteses{pos_inicial: i as i32, pos_final: -1, nivel: nivel});//Adiciona um parenteses ao vetor com a posição inicial e o nível atual
        }
        else if expressao[i].to_string() == ")" //Encontra um fecha parenteses
        {
            for j in 0..vetor_parenteses.len() //Percorre o vetor de parenteses
            {
                if vetor_parenteses[j].nivel == nivel && vetor_parenteses[j].pos_final == -1 //Encontra o parenteses que abre
                {
                    vetor_parenteses[j].pos_final = i as i32; //Adiciona a posição final ao parenteses que abriu
                    nivel = nivel -1; //Diminui o nível
                }
            }
        }
    }
    expression_analizer(&mut vetor_parenteses, expressao);
}

fn expression_analizer(vetor_parenteses: &mut Vec<Parenteses>, expressao: &mut Vec<String>)
{
    let mut nivel_atual: i32 = 0;

    for i in 0..vetor_parenteses.len()
    {
        if vetor_parenteses[i as usize].nivel > nivel_atual
        {
            nivel_atual = vetor_parenteses[i as usize].nivel; //Recebe o maior nível de parenteses para trabalhar
        }
    }

    let mut check_remove_parenteses = 0;

    let mut exp = vec![];
    for i in 0..vetor_parenteses.len()
    {
        if vetor_parenteses[i].pos_final - vetor_parenteses[i].pos_inicial == 2 //Verifica se o parenteses não tem operações
        {
            for i in 0..vetor_parenteses[i].pos_inicial //Remove os parenteses
            {
                exp.push(expressao[i as usize].to_string());
            }
            exp.push(expressao[vetor_parenteses[i].pos_inicial as usize +1].to_string());
            
            for i in vetor_parenteses[i].pos_final+1..expressao.len() as i32
            {
                exp.push(expressao[i as usize].to_string());
            } 

            add_nivel_to_parenteses(&mut exp); //Após remover os parenteses sem operação, recalcula os níveis
        }

        if vetor_parenteses[i].nivel == nivel_atual //Busca por parenteses de mesmo nível que o removido
        {
            check_remove_parenteses = check_remove_parenteses +1;
        }
    }

    if check_remove_parenteses == 0 //Se não encontrar parenteses de mesmo nível, abaixa o nível em 1
    {
        nivel_atual = nivel_atual -1;
    }

    solve_operation(expressao, vetor_parenteses, nivel_atual);
}

fn solve_operation(expressao: &mut Vec<String>, vetor_parenteses: &mut Vec<Parenteses>, nivel_atual: i32)
{
    for i in 0..expressao.len() //Printa a expressão com base no vetor
    {
        print!("{} ", expressao[i]);
    }
    println!();

    let mut nivel_auxiliar = nivel_atual;
    if expressao.len() == 1
    {
        std::process::exit(1); //Se só existe um numero restante, termina a execução
    }

    if vetor_parenteses.len() == 0 //Caso exista operações e não tenha parenteses
    {
        vetor_parenteses.push(Parenteses{pos_inicial:0, pos_final: expressao.len() as i32, nivel: 1});
        nivel_auxiliar = 1; 
        //Adiciona um parenteses "falso"
    }

    let mut operador: char = '\0';

    let mut posicao_auxiliar: i32 = 0;
    let mut x: i32;


    for i in 0..vetor_parenteses.len() //Percorre o vetor de parenteses
    {
        if vetor_parenteses[i].nivel == nivel_auxiliar //Encontra o primeiro parenteses de nivel mais alto
        {
            x = vetor_parenteses[i].pos_inicial; //Recebe a posição inicial desse parenteses

            while x < vetor_parenteses[i].pos_final //Percorre o parenteses até o fim
            {
                if expressao[x as usize].to_string() == "+" || expressao[x as usize].to_string() == "-"
                {
                    if operador == '\0' //Se o operador for vazio recebe o primeiro operador
                    {
                        posicao_auxiliar = x;
                        operador = expressao[x as usize].chars().next().unwrap();
                    }
                }
                else if expressao[x as usize].to_string() == "*" || expressao[x as usize].to_string() == "/" //Se encontrar operadores de maior precedencia
                {
                    if expressao[x as usize].to_string() != "*" || expressao[x as usize].to_string() != "/" //Caso o maior operador encontrado ainda nao seja os de maior precedencia
                    {
                        posicao_auxiliar = x; //Posição do operador de maior precedencia
                        operador = expressao[x as usize].chars().next().unwrap(); //Essa é a operação mais precedente
                        break;
                    }
                }
                x = x + 1;
            }
        }

        let resultado: i32;
        let mut expressao_auxiliar: Vec<String> = Vec::new();

        match operador  //Resolve a operação de acordo com o operador mais precedente encontrado
        {
            '*' =>
            {
                let operando1 = expressao.get(posicao_auxiliar as usize -1).unwrap().parse::<i32>().unwrap();
                let operando2 = expressao.get(posicao_auxiliar as usize +1).unwrap().parse::<i32>().unwrap();

                resultado = operando1 * operando2; // Resolve a expressão

                for i in 0..posicao_auxiliar-1     //Atualiza a expressão passando os novos valores pra um vetor auxiliar
                {
                    expressao_auxiliar.push(expressao[i as usize].to_string());
                }

                expressao_auxiliar.push(resultado.to_string());
                
                for i in posicao_auxiliar+2..expressao.len() as i32
                {
                    expressao_auxiliar.push(expressao[i as usize].to_string());
                }

                add_nivel_to_parenteses(&mut expressao_auxiliar); //Recalcula os níveis dos parenteses novamente
            },

            '/' =>
            {
                let operando1 = expressao.get(posicao_auxiliar as usize -1).unwrap().parse::<i32>().unwrap();
                let operando2 = expressao.get(posicao_auxiliar as usize +1).unwrap().parse::<i32>().unwrap();

                resultado = operando1 / operando2;

                for i in 0..posicao_auxiliar-1
                {
                    expressao_auxiliar.push(expressao[i as usize].to_string());
                }

                expressao_auxiliar.push(resultado.to_string());
                
                for i in posicao_auxiliar+2..expressao.len() as i32
                {
                    expressao_auxiliar.push(expressao[i as usize].to_string());
                }

                add_nivel_to_parenteses(&mut expressao_auxiliar);
            },

            '+' =>
            {
                let operando1 = expressao.get(posicao_auxiliar as usize -1).unwrap().parse::<i32>().unwrap();
                let operando2 = expressao.get(posicao_auxiliar as usize +1).unwrap().parse::<i32>().unwrap();

                resultado = operando1 + operando2;
                for i in 0..posicao_auxiliar-1
                {
                    expressao_auxiliar.push(expressao[i as usize].to_string());
                }

                expressao_auxiliar.push(resultado.to_string());
                
                for i in posicao_auxiliar+2..expressao.len() as i32
                {
                    expressao_auxiliar.push(expressao[i as usize].to_string());
                }

                add_nivel_to_parenteses(&mut expressao_auxiliar);
            }
            ,

            '-' => 
            {
                let operando1 = expressao.get(posicao_auxiliar as usize -1).unwrap().parse::<i32>().unwrap();
                let operando2 = expressao.get(posicao_auxiliar as usize +1).unwrap().parse::<i32>().unwrap();

                resultado = operando1 - operando2;

                for i in 0..posicao_auxiliar-1
                {
                    expressao_auxiliar.push(expressao[i as usize].to_string());
                }

                expressao_auxiliar.push(resultado.to_string());
                
                for i in posicao_auxiliar+2..expressao.len() as i32
                {
                    expressao_auxiliar.push(expressao[i as usize].to_string());
                }

                add_nivel_to_parenteses(&mut expressao_auxiliar);
            }
            ,

            _=> print!("")
        }
    }
}