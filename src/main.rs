use std::io;

fn main()
{
    let fechados = ['}', ']', ')'];
    let abertos = ['{', '[', '('];

    let index_of = |busca, onde: [char; 3]| { let pos = onde.iter().position(|&x| x == busca); if pos==None { -1 as isize } else {pos.unwrap() as isize}};


    let mut buffer = String::new();
    let stdin = io::stdin(); // Acesso ao console

    print!("Digite a sequencia: ");

    let _qt = stdin.read_line(&mut buffer); //_ na frente da variável porque não será usada
    let mut pilha = Vec::new();

    let mut seqvalida = true;

    for c in buffer.trim().chars()
    {
        match c
        {
            '{' | '[' | '(' =>  {    //println!("Aberto: {}", c);
                                    pilha.push(c);
                                }
            '}' | ']' | ')' =>  {   //println!("Fechado: {}", c); 
                                    let aberto = abertos[index_of(c, fechados) as usize];

                                    if let Some(x) = pilha.last() {
                                        if *x==aberto
                                        {
                                            pilha.pop();
                                        }
                                        else
                                        {
                                            println!("Sequencia invalida!");
                                            seqvalida = false;
                                            break;
                                        }
                                    }
                                }
            _ =>                {   println!("Caracter invalido na sequencia!");
                                    seqvalida = false;
                                    break;
                                }
        }
    }

    if seqvalida
    {
        println!("A sequencia eh valida!");
    }

}
