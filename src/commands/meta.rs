use serenity::{
    model::channel::Message,
    prelude::*,
};
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use std::thread;
use std::time::Duration;

use psutil::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult { 
    let bot = &ctx.cache.current_user().await;
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("About Carberretta");
            e.author(|a| {
                a.name("Carberretta");

                a
            }); 
            e.description("Type `+info` for bot stats.");
            e.color(0x00cD99);
            e.thumbnail(&bot.face());
            e.fields(vec![
                ("Authors", "Not Complete", false),
                ("Source", "Click [here](https://github.com/Carberra/Carberretta)", false),
                ("License", "[BSD 3-Clause](https://github.com/Carberra/Carberretta/blob/master/LICENSE)", false)
            ]);
            e.footer(|f| {
                f.text(format!("Requested by {}", &msg.author.name));
                f.icon_url(&msg.author.face());

                f
            });

            e
        });
        m
    }).await;

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn botinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let bot = &ctx.cache.current_user().await;
    let precision = 3;
    let mib = 1024_u64;
    let block_time = Duration::from_millis(1000);
    let uptime = host::uptime().unwrap();
    let virtual_memory = memory::virtual_memory().unwrap();
    let processes = process::processes().unwrap();
    let disk_usage = disk::disk_usage("/").unwrap();
    let mut cpu_percent_collector = cpu::CpuPercentCollector::new().unwrap();
    let mut cpu_times_percent_collector = cpu::CpuTimesPercentCollector::new().unwrap();

    let mem_total = (&virtual_memory.total() / mib.pow(2)).to_string();
    let mem_usage = (&virtual_memory.used() / mib.pow(2)).to_string();
    let mem_percent = &virtual_memory.percent().to_string();

    // let mem_total: &str = &*mem_total_raw;
    // let mem_usage: &str = &*mem_usage_raw;
    // let mem_percent: &str = &*mem_percent_raw;

    println!("UPTIME INFORMATION: {:?}", &uptime);
    println!("VIRTUAL MEMORY USED: {:.1$}", &mem_usage, &precision);
    println!("VIRTULAL MEMORY TOTAL: {}", &mem_total);
    println!("VIRTUAL MEMORY PERCENT: {}", &mem_percent);
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Carberretta Information");
            e.author(|a| {
                a.name("Carberretta");

                a
            });    
            e.color(0x00cD99);
            e.thumbnail(&bot.face());
            e.fields(vec![
                ("Bot Version", "INSERT VALUE", true),
                ("Rust Version", "INSERT VALUE", true),
                ("Serenity Version", "INSERT VALUE", true),
                ("Uptime", "INSERT HERE", true),
                ("CPU Time", "INSERT HERE", true),
                ("Memory Usage", &format!("{} / {} MiB ({:.3}%)", mem_usage, mem_total, mem_percent), true),
                ("Code Lines", "INSERT HERE", true),
                ("Docs Lines", "INSERT HERE", true),
                ("Blank Lines", "INSERT HERE", true),
                ("Database Calls", "INSERT HERE", true), 
            ]);
            e.footer(|f| {
                f.text(format!("Requested by {}", &msg.author.name));
                f.icon_url(&msg.author.face());

                f
            });
            
            e
        });
        m
    }).await;

    if let Err(why) = msg{
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
