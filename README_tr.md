## Proje Tanımı
Bu proje, Rust kütüphaneleri olan `egui` ve `eframe` kullanılarak platformlar arası (native ve WASM/web) bir masaüstü uygulaması oluşturur. Uygulama, bir SQLite veritabanından (`Northwind.db`) müşteri verilerini alır ve bunları bir kullanıcı arayüzünde gösterir.

#### Temel Özellikler:

- **Veritabanı Erişimi:**
Native tarafta veriler `reqwest` (senkron) kullanılarak alınır, WASM tarafında ise tarayıcının Fetch API'si (asenkron) ile `http://localhost:3000/customers` adresindeki API endpoint'inden JSON formatında müşteri verileri çekilir.  
Bu API endpoint’i, ya `server/index.ts` dosyası üzerinden Bun ve `bun:sqlite` kullanılarak ya da `server/main.ts` dosyasında Deno ve `jsr:@db/sqlite` kullanılarak sunulur. Her iki sunucu da `db/Northwind.db` SQLite dosyasına erişerek sorgulama yapar.

- **Kullanıcı Arayüzü (egui):**
`sqlitedata.rs` dosyasında, alınan müşteri verileri hem ham JSON formatında hem de `egui_extras::TableBuilder` kullanılarak yapılandırılmış bir tablo şeklinde gösterilir.  
"Customer Name" alanına tıklandığında, ilgili müşterinin ID’si tablonun altında renkli bir etiketle görüntülenir.  
Veri çekme işlemi, pencere ilk açıldığında otomatik olarak tetiklenir ve ayrıca "Fetch Customer Data" butonuna tıklanarak manuel olarak da gerçekleştirilebilir.  
`info.rs` dosyasında basit bir "Hakkında" penceresi yer alır.

- **Platform Desteği:**
Proje, `src/main.rs` üzerinden native masaüstü uygulaması olarak derlenip çalıştırılabilir veya `run_httpserver_with_*.bat` dosyaları kullanılarak bir web tarayıcısında WebAssembly (WASM) uygulaması olarak çalıştırılabilir.  
Koşullu derleme (`#[cfg(...)]`) kullanılarak native ve WASM hedefleri için farklı HTTP istemci implementasyonları sağlanır.

**Özetle:** Bu proje, Rust içinde `egui` kullanarak basit bir CRUD benzeri (Create, Read, Update, Delete – burada yalnızca Read uygulanmıştır) uygulamayı örneklemektedir. Bir API aracılığıyla veritabanından veri okur ve bunu kullanıcı dostu tablo formatında sunar. Uygulama hem masaüstü hem de web platformlarında çalışabilir.

## Server Çalıştırma
**Bun** yüklü olmak olmak üzere `server` dizininde aşağıdaki komut çalıştırılır. `index.ts` betiği Northwind veritabanındaki müşteri verilerini `/customers` yolu üzerinden JSON olarak sunan minimal bir API sunucusudur:
```bash
bun run index.ts
```

## Native veya WASM derleme

### Native derleme
```bash
cargo clean
cargo update
cargo build
cargo run
```

### WASM olarak derleyip tarayıca çalıştırmak için gerekli adımlar
WASM hedefi: Rust'ın WASM'a derleme yapabilmesi için wasm32-unknown-unknown hedefini yükleyin:
```bash
rustup target add wasm32-unknown-unknown
```

wasm-pack: Rust kodunuzu WebAssembly'e paketlemek ve JavaScript ile uyumlu hale getirmek için wasm-pack aracını yükleyin:
```bash
cargo install wasm-pack
```
Projenizin ana dizininde aşağıdaki komutu çalıştırın:

```bash
wasm-pack build --target web --out-name emartident_rust_wasm --out-dir ./dist/
```
Bu komut:

*--target web*: Tarayıcı ortamları için çıktı üretir.
*--out-name wasm*: Oluşturulacak .wasm ve .js dosyalarının adını wasm olarak belirler (örn: wasm.js, wasm_bg.wasm).
*--out-dir ./dist/*: Çıktı dosyalarını projenizin ana dizininde dist adlı bir klasöre yerleştirir.

Derlenen WASM modülünü yükleyecek bir HTML dosyasına ihtiyacınız var. Projenizin ana dizinine aşağıdaki içerikle *index.html* adında bir dosya oluşturun:

```html
<!DOCTYPE html>
<html lang="tr">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WASM Application</title>
    <style>
        /* Canvas'ın tüm alanı kaplaması ve kaydırma çubuklarını önlemesi için temel stiller */
        html, body {
            height: 100%;
            margin: 0;
            overflow: hidden;
            background-color: #30303000; /* Yükleme sırasında koyu arka plan */
        }
        canvas {
            display: block; /* Satır içi blok boşluğunu kaldır */
            width: 100%;
            height: 100%;
        }
    </style>
</head>
<body>
    <!-- WASM kodu bu canvas'ı yeniden boyutlandıracak ve uygulamayı burada gösterecek -->
    <canvas id="the_canvas_id"></canvas>

    <!-- wasm-pack tarafından oluşturulan JavaScript dosyası -->
    <script type="module">
        // './dist/emartident_rust_wasm.js' yolu, wasm-pack çıktınıza göre ayarlanmalıdır.
        import init, { start } from './dist/emartident_rust_wasm.js';

        async function run() {
            // Önce Wasm modülünü yükle
            await init();

            // lib.rs dosyasından dışa aktarılan 'start' fonksiyonunu çağır.
            start('the_canvas_id');
        }

        run();
    </script>
</body>
</html>
```

Oluşturulan dosyaları (*index.html* ve *dist* klasörü) bir web sunucusu aracılığıyla sunmanız gerekir. Projenizin ana dizininde basit bir HTTP sunucusu başlatabilirsiniz.

Eğer **Python** yüklüyse:
```bash
# Eğer tüm ağ arayüzlerinden erişilebilir olmasını istiyorsanız (dikkatli olun, bu güvenlik riski oluşturabilir)
python -m http.server 8080 --bind 0.0.0.0
# veya
python -m http.server 8080 --bind 127.0.0.1
```

Eğer **Deno** yüklüyse:
```bash
deno run --allow-net --allow-read jsr:@std/http/file-server --port 8080
# veya
deno run --allow-net --allow-read jsr:@std/http/file-server --addr 0.0.0.0:8080
```

Eğer **Bun** yüklüyse:

```bash
bunx http-server . -a 0.0.0.0 -p 8080
```

Ardından tarayıcınızda *http://localhost:8080* (veya sunucunun kullandığı port) adresini açarak uygulamanızı görebilirsiniz. Yerel ağdaki diğer cihazdan da server eden cihaz IP bilgisi girerek uygulamaya erişilebilir.

Bu adımlarla projeniz web tarayıcısında çalışır hale gelecektir.