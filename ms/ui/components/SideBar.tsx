interface Props {
    isOpen: boolean;
  }
  
  export default function SideBar({ isOpen }: Props) {
    return (
      <aside
        style={{
          position: "fixed",
          top: 0,
          left: 0,
          height: "100vh",
          width: "16rem",
          backgroundColor: "#4a5568",
          color: "white",
          transition: "transform 0.3s ease-in-out",
          transform: isOpen ? "translateX(0)" : "translateX(-100%)",
          display: "block",
        }}
      >
        <div style={{ padding: "1rem", marginTop: "3rem" }}>
          <h2 style={{ fontSize: "1.25rem", fontWeight: "bold" }}>
            Menu
          </h2>
          <ul style={{listStyle: 'none', padding: 0}}>
            <li style={{ marginTop: "1rem" }}>
              <a href="#" style={{ color: "white", textDecoration: "none" }}>
                古参登録画面
              </a>
            </li>
            <li style={{ marginTop: "1rem" }}>
              <a href="#" style={{ color: "white", textDecoration: "none" }}>
                古参ランキング
              </a>
            </li>
            <li style={{ marginTop: "1rem" }}>
              <a href="#" style={{ color: "white", textDecoration: "none" }}>
                設定
              </a>
            </li>
          </ul>
        </div>
        {/* Media query handling for larger screens */}
        {(typeof window !== 'undefined' && window.innerWidth >= 768) && (
          <style jsx global>{`
            aside {
              display: none;
            }
          `}</style>
        )}
      </aside>
    );
  }