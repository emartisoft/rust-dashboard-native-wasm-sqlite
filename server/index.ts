import { Database } from "bun:sqlite";
import { existsSync, mkdirSync } from "node:fs"; // Dosya sistemi işlemleri için

// Veritabanı dosyasının yolu
const dbFolderPath = "./db";
const dbFilePath = `${dbFolderPath}/Northwind.db`;

// db klasörünün var olduğundan emin ol (Northwind.db dosyasının bu klasörde olması beklenir)
if (!existsSync(dbFolderPath)) {
  mkdirSync(dbFolderPath, { recursive: true });
  console.log(`Created directory: ${dbFolderPath}`);
}

if (!existsSync(dbFilePath)) {
  console.error(
    `Error: Database file not found at ${dbFilePath}. Please ensure Northwind.db exists in the db folder.`,
  );
  process.exit(1); // Veritabanı dosyası yoksa uygulamayı sonlandır
}

const db = new Database(dbFilePath);

console.log("Bun server running on http://localhost:3000");
console.log("Access customers at http://localhost:3000/customers");

Bun.serve({
  port: 3000,
  fetch(req) {
    const url = new URL(req.url);
    if (url.pathname === "/customers") {
      try {
        const query = `
          SELECT 
            [Customers].[CustomerName], 
            [Customers].[Address], 
            [Customers].[CustomerID]
          FROM   [Customers] WHERE [Customers].[CustomerID]<8;
        `;
        const customers = db.query(query).all();
        return new Response(JSON.stringify(customers), {
          headers: { 
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "*", // Bu başlık headers nesnesinin içinde olmalı
        },
        });
      } catch (error) {
        console.error("Error fetching customers:", error);
        return new Response(
          JSON.stringify({
            error: "Failed to retrieve customers",
            details: error instanceof Error ? error.message : String(error),
          }),
          { 
            status: 500, 
            headers: { 
                "Content-Type": "application/json",
                "Access-Control-Allow-Origin": "*",
            } 
        },
        );
      }
    }
    return new Response("Not Found", { 
        status: 404 ,
        headers: { "Access-Control-Allow-Origin": "*" },
    });
  },
});