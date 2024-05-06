#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Santiago,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn wine_popularity(w: WineRegions) {
    match w {
        WineRegions::Bordeaux | WineRegions::Burgundy | WineRegions::Champagne => println!("Highly popular."),
        WineRegions::Tuscany => println!("Popular"),
        _ => println!("Not popular.")
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3: Wine = Wine {
        name: String::from("Cabernet Sauvignon"),
        region: WineRegions::Santiago,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);

    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);

    wine_popularity(WineRegions::Bordeaux);
    wine_popularity(WineRegions::Tuscany);
    wine_popularity(WineRegions::Santiago);
}