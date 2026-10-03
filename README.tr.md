# 🚀 ClubChains

> English version: [README.md](README.md)

**ClubChains**, kulüp içi üyelik, oy verme, etkinlik yönetimi ve finansal işlemleri takip eden bir otomasyon sistemidir. Stellar Soroban akıllı kontrat platformuna entegre edilerek zincir üstü güvenli işlemler sağlar.

## 🛠 Özellikler

### ✅ Üyelik & Oy Sistemi
- Üye kaydı
- Oy kullanma
- Oy hakkı devretme (delegasyon)
- Üyeyi engelleme

### 💬 Etkileşim & Raporlama
- Etki skoru raporu
- Üye listesi ve oy kayıtları
- Etkinlik oluşturma ve listeleme
- Sertifika verme ve listeleme

### 💰 Finansal İşlemler
- Gelir ekleme
- Gider ekleme
- Finansal rapor görüntüleme

### 🪙 Soroban Token Entegrasyonu (Zincir Üstü)
- Token mint (zincir üstü)
- Token burn (zincir üstü)
- Stellar Testnet destekli kontrat çağrıları (invoke)

## 📦 Proje Yapısı

```
clubchains/
├── src/
│   ├── logic/           # İş kuralları (mint, burn, vote, delegate...)
│   ├── contracts/       # Soroban kontratları (.wasm)
│   ├── menu.rs          # CLI ana menü
│   ├── models.rs        # Veri modelleri
│   ├── storage.rs       # JSON veri kaydı
│   └── main.rs          # Program başlangıcı
├── Cargo.toml
└── README.md
```

## 🧪 Kullanım

```bash
cargo run
```

CLI menüsünden interaktif olarak tüm işlemleri gerçekleştirebilirsiniz.

## 🔗 Soroban Entegrasyonu

### .wasm Oluşturma

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Kontratı Testnet’e Deploy Et

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/clubchains.wasm \
  --source fatih276 \
  --network testnet
```

### Token Mint Örneği
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source fatih276 \
  --network testnet \
  -- mint \
  --to <USER_ADDRESS> \
  --amount 100
```

> NOT: `--source` için kimliğinizin Soroban CLI ile oluşturulmuş olması gerekir:
> ```bash
> stellar keys generate fatih276 --network testnet
> ```

## 💼 Geliştirici Bilgisi

Proje, **Rise In x Patika.dev x Stellar Rust Hackathon 2025** kapsamında geliştirilmiştir.

Geliştirici: **Fatih Erdoğan**  
GitHub: [github.com/FatihErdogan1](https://github.com/FatihErdogan1)

---

## 📃 Lisans

MIT License
