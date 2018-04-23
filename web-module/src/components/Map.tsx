import * as React from 'react';
import { CellButton } from './CellButton';
import { Stage, Layer } from 'react-konva';

export class Map extends React.Component<any, any> {

  constructor (props: any) {
    super(props);
    this.state = { cellStatus : {} }

    for (let i = 0; i < this.props.side; i++) {
      for (let i2 = 0; i2 < this.props.side; i2++) {
        const key = `${i}-${i2}`;
        this.state.cellStatus[key] = false;
      }
    }
  }

  clickCell = (cell: any) => {
    const key = cell._reactInternalFiber.key;
    this.setState((prevState: any) => {
      return {
        cellStatus: { ...prevState.cellStatus, [key]: !prevState.cellStatus[key] }
      }
    }, () => {
      console.log(this.state.cellStatus);      
    });
  }

  render () {
    
    const matrix = [];

    const cellSide = ((window.innerHeight - 30) / this.props.side);

    console.log(cellSide);

    for (let x = 0; x < this.props.side; x++) {
      for (let y = 0; y < this.props.side; y++) {
        const key = `${x}-${y}`;
        matrix.push(<CellButton key={key} onClick={this.clickCell} side={cellSide} x={x * cellSide} y={y * cellSide} />);
      }
    }

    return (
      <Stage width={window.innerHeight - 30} height={window.innerHeight - 30}>
        <Layer>
          {matrix}
        </Layer>
      </Stage>
    );
  }
}
