use std::io;

fn main()
{
    //let abertos = vec!['{', '[', '(']; //Macro para preenchimento
    let _fechados = ['}', ']', ')'];
    let _abertos = ['{', '[', '('];

    let index_of = |busca, onde: [char; 3]| { let pos = onde.iter().position(|&x| x == busca); if pos==None { -1 as isize } else {pos.unwrap() as isize}};
    let _pos1 = index_of('[', _abertos);

//    println!("{:?}", _fechados[pos1 as usize]);

    let mut buffer = String::new();
    let stdin = io::stdin(); // Acesso ao console
    let _qt = stdin.read_line(&mut buffer); //_ na frente da variável porque não será usada
    let mut pilha = Vec::new();
    
    for c in buffer.trim().chars()
    {
        println!("Char: {}", c);
        match c
        {
            '\n' => {continue},
            _ => {}
        };

        if index_of(c, _abertos)>-1
        {
            pilha.push(c);
        }
        else
        {
            let pos = index_of(c, _fechados);
            if pos>-1
            {
                let aberto = _abertos[pos as usize];

                if let Some(x) = pilha.last() {
                    if *x==aberto
                    {
                        pilha.pop();
                    }
                    else
                    {
                        println!("Sequencia invalida!");
                        break;
                    }
                }
                /*
                if option.is_some()
                {
                    let x = option.unwrap();

                    if *x==aberto
                    {
                        println!("{} == {}", aberto, *x);
                        pilha.pop();
                    }
                }
                */
            }
            else
            {
                println!("Caracter invalido!");
                break;
            }
        }
        /*
        let result = match c
        {
            c if index_of(c, _abertos)>-1 => {println!("foi!"); pilha.push(c);},
            x if index_of(c, _fechados)>-1 => {println!("{}", x);},
            _ => println!("outra coisa")
        };
        println!("Result: {:?}", result);
        */
    }

}
