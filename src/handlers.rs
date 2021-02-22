use std::vec::Vec;

pub fn number_to_words(number:u128) -> String {
    
    #[derive(Debug)]
    struct Figures {
        unid: u128,
        dec: u128,
        cent: u128,
        um: u128,
    }
    
    //Traducciones principales
    let unidades:[&str; 10] = ["cero", "uno", "dos" ,"tres" ,"cuatro" ,"cinco" ,
    "seis" ,"siete" ,"ocho" ,"nueve"];
    let especiales:[&str; 10] = ["diez","once", "doce","trece","catorce", "quince", 
    "diezciseis", "diecisiete", "dieciocho", "diecinueve"];
    let decenas:[&str; 8] = ["veinte", "treinta","cuarenta","cincuenta", "sesenta",
    "setenta", "ochenta", "noventa"];
    let centenas:[&str; 9] = ["ciento", "docientos","trecientos","cuatrocientos", "quinientos",
    "seicientos", "setecientos", "ochocientos", "novecientos"];
    let mmillon:[&str; 20] = ["millòn","billón", "trillón","cuatrillón","quintillón","sextillón","septillón",
    "octillón","nonillón", "decillón", "undecillón", "duodecillón", "tredecillón", "cuatordecillón", 
    "quindecillón", "sexdecillón", "septendecillón", "octodecillón", "novendecillón", "vigintillón"];
    let mmillones:[&str; 20] = ["millones", "billones", "trillones","cuatrillones","quintillones","sextillones","septillones",
    "octillones","nonillones", "decillones", "undecillones", "duodecillones", "tredecillones", "cuatordecillones", 
    "quindecillones", "sexdecillones", "septendecillones", "octodecillones", "novendecillones", "vigintillones"];
 
    fn decompose (number:u128)-> Figures {
        let mut num:u128 = number;
        let unid = num % 10;
        num = num/10;
        let dec = num%10;
        num = num/10;
        let cent = num%10;
        let um = num/10;
        
        let desc = Figures{
            unid:unid,
            dec:dec,
            cent:cent,
            um:um
        };
        return desc;
    } 
    
    fn convertirUnidad(unidades:[&str; 10],unid:usize)->&str {
        let response = unidades[unid];
        return response;
    }
    
    fn convertirDecena(dec:usize,unid:usize,millones:bool,unidades:[&str; 10],decenas:[&str; 8])->String {
      let mut response:String = "".to_owned();
       
        if unid == 0 {
            response.push_str(decenas[dec-2]);               
        }else{
            if dec >= 3 {
                if millones && unid==1 {
                    response.push_str(decenas[dec-2]);
                    response.push_str("y un");
                    
                }else{
                    response.push_str(decenas[dec-2]);
                    response.push_str(" y ");
                    response.push_str(unidades[unid]);
                }
            }else{
                
                if millones && unid==1 {
                    response.push_str("veintiún");
                    
                }else{
                    response.push_str("veinti");
                    response.push_str(unidades[unid]);
                }
            }
            
        }
        
        return response;
    } 
    
    fn convertCentena(num:u128,millones:bool,unidades:[&str; 10],decenas:[&str; 8],centenas:[&str; 9],especiales:[&str; 10])->String {
    
        let mut response:String = "".to_owned();
        
        let desc = decompose(num);
        let cent = desc.cent;
        let dec = desc.dec;
        let unid = desc.unid;

        if num>=0 && num<10{
            response.push_str(convertirUnidad(unidades,num as usize));         
        }else if num<20{
            response.push_str(especiales[(num-10)  as usize]);        
        }else if num<100 {
            response.push_str(&(convertirDecena(dec as usize,unid as usize,millones,unidades,decenas)));
        }else {
        
            if num == 100 {
                response.push_str("cien");
            }else if num %100 == 0 {
                response.push_str(centenas[(cent-1) as usize]);
            }else{
                if dec==0 {
                    response.push_str(centenas[(cent-1) as usize]);
                    response.push_str(" ");
                    response.push_str(convertirUnidad(unidades,unid as usize));
                }else if dec==1 { 
                    response.push_str(centenas[(cent-1) as usize]);
                    response.push_str(" ");
                    response.push_str(especiales[unid as usize]);
                }else{
                    response.push_str(centenas[(cent-1) as usize]);
                    response.push_str(" ");
                    response.push_str(&(convertirDecena(dec as usize,unid as usize,millones,unidades,decenas)));
                }
                
            }
        }     
        
        return response;
    }

    fn reverse(array: Vec<&str>) -> Vec<&str>{
        
        let mut reverso:Vec<&str>=Vec::new();
        for i in array.iter().rev() {
            reverso.push(i);
        }
        return reverso;
    }
    
    fn convertStringToVector(number:String)->String {
        
        
        let mut x: Option<f32> = None;
        let mut entrada = number.split_terminator("").skip(1).collect::<Vec<&str>>();
        entrada = reverse(entrada);

        let mut salida:Vec<String>=Vec::new();
        let mut aux:String = "".to_owned();
        let outputSize:f64= entrada.len() as f64;
        let divider:f64 = 3.0;
        let round:f64= outputSize / divider;
       
        let paginador:u32 = round.ceil() as u32;
        
        println!("paginador: {}",paginador);
        for i in 0..(paginador as usize ) {
            
            for j in 0..3 {
                aux.push_str(entrada[j + (i*3)]);
            }    
            salida.push(aux.clone());
            aux = "".to_string();
            
        }
        
        let mut joined = salida.join(".");
        return joined;
    }

    
    
    let joined = convertStringToVector(number.to_string());

    let separator = joined.split_terminator("").skip(1).collect::<Vec<&str>>();
    let reversed = reverse(separator);
    let join = reversed.join("");
    let mut vector = join.split_terminator(".").skip(0).collect::<Vec<&str>>();
    
    let mut response:String = "".to_owned(); 
    if vector.len() == 1 {
        response.push_str(&convertCentena(number, false, unidades, decenas, centenas, especiales));
    }else if vector.len() == 2 {
    
        if vector[1]=="000" {
            if  vector[0].parse::<i32>().unwrap() == 1 {
                response.push_str(" mil " );
            }else{
                response.push_str(&convertCentena(vector[0].parse::<u128>().unwrap() , false, unidades, decenas, centenas, especiales));
                response.push_str(" mil ");
            }
        
        }else{
            
            if  vector[0].parse::<i32>().unwrap() == 1 {
                response.push_str(" mil " );
                response.push_str(&convertCentena(vector[1].parse::<u128>().unwrap() , false, unidades, decenas, centenas, especiales));
            }else{
                response.push_str(&convertCentena(vector[0].parse::<u128>().unwrap() , false, unidades, decenas, centenas, especiales));
                response.push_str(" mil " );
                response.push_str(&convertCentena(vector[1].parse::<u128>().unwrap() , false, unidades, decenas, centenas, especiales));
            }
        }
    }else{
        
        let mut complete:Vec<String>=Vec::new();
        vector = reverse(vector);

        for i in 0..(vector.len() as usize ) {
            
            for j in i+1..i+2 {
                
                if i%2 == 1 {
                    if vector[i].parse::<i32>().unwrap() ==1 {
                        complete.push(" mil ".to_string());
                    }else if vector[i].parse::<i32>().unwrap() > 1 {
                        let mut aux:String = "".to_owned(); 
                        aux.push_str(&convertCentena(vector[i].parse::<u128>().unwrap() , true, unidades, decenas, centenas, especiales));
                        aux.push_str(" mil ");
                        complete.push(aux); 
                    }
                    
                }else{
                    if i==0 && vector[i].parse::<u128>().unwrap() !=0 {
                        complete.push(convertCentena(vector[i].parse::<u128>().unwrap() , true, unidades, decenas, centenas, especiales));
                    }else{
                        if vector[i].parse::<u128>().unwrap() ==1 {
                            let mut aux:String = "".to_owned(); 
                            aux.push_str(" un "); 
                            aux.push_str(mmillon[(i/2)-1]);
                            complete.push(aux); 
                            
                        }else if  vector[i].parse::<u128>().unwrap() >1 {
                            let mut aux:String = "".to_owned();
                            aux.push_str(&convertCentena(vector[i].parse::<u128>().unwrap() , true, unidades, decenas, centenas, especiales));
                            aux.push_str(" "); 
                            aux.push_str(mmillones[(i/2)-1]);
                            aux.push_str(" "); 
                            complete.push(aux);
                        }else if  vector[i].parse::<u128>().unwrap() ==0 && vector[j].parse::<u128>().unwrap() !=0 {
                                complete.push(mmillones[(i/2)-1].to_string()); 
                        }
                    }
                }
            }
        }
        
        let mut reverso:Vec<&str>=Vec::new();
        for i in complete.iter().rev() {
            reverso.push(i);
        }
        
        let joinComplete = reverso.join("");
        response = joinComplete;
    }
        
    return response;
    
    
}
