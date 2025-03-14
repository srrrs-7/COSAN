import { JSX } from "preact";
import { useSignal } from "@preact/signals";

export default function RegisterWord() {
  const word = useSignal("");

  const handleSubmit = (e: JSX.TargetedEvent<HTMLFormElement>) => {
    e.preventDefault();
    
    console.log("Word:", word);

    // ここで登録処理などを実行
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center", color: "black" }}>
      <form
        onSubmit={handleSubmit}
        style={{ display: "flex", flexDirection: "column", gap: "8px" }}
      >
        
        <div>
          <input
            style={{ 
              width: "250px", 
              height: "30px", 
              padding: "4px", 
              border: "4px solid #6aac98", 
              borderRadius: "10px",
              color: "black",
            }}
            placeholder="人物、モノ、場所、商品名、フレーズなど"
            type="text"
            id="word"
            value={word.value}
            onInput={(e: JSX.TargetedEvent<HTMLInputElement>) => {
              word.value = e.currentTarget.value;
            }}
            required
          />
        </div>

        <button
          style={{ 
            width: "100px", 
            height: "30px", 
            padding: "2px", 
            border: "none", 
            backgroundColor: "#6aac98", 
            cursor: "pointer", 
            alignSelf: "center",
            borderRadius: "10px",
          }}
          type="submit"
        >
          登録
        </button>

      </form>

      <div>
        ranking
      </div>
    </div>
  );
}