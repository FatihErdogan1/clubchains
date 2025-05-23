ClubChain, üniversite kulüpleri için geliştirilen, token tabanlı, Stellar esinli bir topluluk otomasyon sistemidir. Rust diliyle yazılmıştır ve kulüp içi işlemleri şeffaf, denetlenebilir ve katılımcı hale getirmeyi hedefler.

🚀 Özellikler

🎓 Üyelik kaydı ve listeleme

🗳️ Oy kullanma, oy devri ve engelleme (delegation/freeze)

💰 Token yönetimi: mint ve burn (oy gücü arttır/azalt)

📅 Etkinlik oluşturma ve listeleme

📜 Sertifika verme ve listeleme

💼 Gelir-gider takibi ve finansal raporlama

📊 Etkinlik ve sertifika raporları

🧪 Demo verisi yükleme ile hızlı test

🏗 Proje Yapısı

clubchain/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── menu.rs
    ├── models.rs
    ├── storage.rs
    ├── demo.rs
    └── logic/
        ├── mod.rs
        ├── join.rs
        ├── vote.rs
        ├── delegate.rs
        ├── freeze.rs
        ├── mint.rs
        ├── burn.rs
        ├── event.rs
        ├── income.rs
        ├── expense.rs
        ├── certificate.rs
        ├── financialreport.rs
        └── report.rs

⚙️ Kurulum ve Çalıştırma

cargo build
cargo run

🧪 Demo Modu

İlk testler için menüden 15. Demo Verileri Yükle seçeneğini kullanın. Bu işlem:

5 üyeyi

1 oyu

1 etkinliği

1 sertifikayı

1 gelir + 1 gideri otomatik olarak oluşturur.

📂 Kaydedilen Veri

Tüm veriler chain.json dosyasında saklanır. Program kapansa bile veri korunur.

🧭 Menüden Yapılabilecekler

1. Üye Kaydı
2. Oy Kullan
3. Oy Hakkı Devret (Delegasyon)
4. Kullanıcıyı Engelle
5. Etki Skoru Raporu
6. Üye Listesi
7. Oy Kayıtları
8. Token Ver (Mint)
9. Token Eksilt (Burn)
10. Etkinlik Oluştur
11. Gelir Ekle
12. Gider Ekle
13. Sertifika Ver
14. Finansal Rapor
15. Demo Verileri Yükle
16. Etkinlik Listesi
17. Sertifika Listesi
0. Çıkış

📄 Lisans

MIT Lisansı
