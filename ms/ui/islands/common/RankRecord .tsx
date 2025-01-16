interface Props {
    lastName: string;
    firstName: string;
    registerDate: string;
  }
  
  export default function RankRecord({ lastName, firstName, registerDate }: Props) {
    return (
      <div>
        <h2>ランキング</h2>
        <table>
          <thead>
            <tr>
              <th>順位</th>
              <th>姓</th>
              <th>名</th>
              <th>登録日</th>
            </tr>
          </thead>
          <tbody>
            <tr key={`${data.lastName}-${data.firstName}-${data.registerDate}`}>
              <td>{data.rank}</td>
              <td>{data.lastName}</td>
              <td>{data.firstName}</td>
              <td>{data.registerDate}</td>
            </tr>
          </tbody>
        </table>
      </div>
    );
  }