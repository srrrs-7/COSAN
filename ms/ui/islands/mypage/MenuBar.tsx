import { JSX } from "preact";
import { useState } from "preact/hooks";

export default function MenuBar() {
  const [activeTab, setActiveTab] = useState("search"); // 初期タブ

  const tabs = [
    { id: "search", icon: "/logo.svg", label: "検索" },
    { id: "collection", icon: "/logo.svg", label: "コレクション" },
    { id: "register", icon: "/logo.svg", label: "登録" },
    { id: "community", icon: "/logo.svg", label: "コミュニティ" },
    { id: "login", icon: "/logo.svg", label: "ログイン" },
  ];

  return (
    <div style={{ 
      position: "fixed", 
      bottom: 0, 
      width: "100%", 
      backgroundColor: "#f0f0f0",
      padding: "10px",
      display: "flex",
      justifyContent: "space-around" 
    }}>
      {tabs.map(tab => (
        <div 
          key={tab.id} 
          style={{ 
            display: "flex", 
            flexDirection: "column", 
            alignItems: "center",
            cursor: "pointer" 
          }}
          onClick={() => setActiveTab(tab.id)}
        >
          <img 
            src={tab.icon} 
            alt={tab.label} 
            style={{ 
              width: "30px", 
              height: "30px",
              filter: activeTab === tab.id ? 'brightness(100%) saturate(100%) invert(44%) sepia(61%) saturate(650%) hue-rotate(100deg) brightness(100%) contrast(100%)' : 'none'
            }} 
          />
          <span style={{ fontSize: "12px", color: activeTab === tab.id ? "#6aac98" : "gray" }}>{tab.label}</span> {/* アクティブ時 */}
        </div>
      ))}
    </div>
  );
};