export const T3_Types = {
  OUTPUT: 0,
  INPUT: 1,
  VARIABLE: 2,
  SCHEDULE: 4,
  HOLIDAY: 5,
  PROGRAM: 6,
  MON: 9,
};

const gaugeDefautColors = [
  { offset: 33, color: "#14BE64" },
  { offset: 66, color: "#FFB100" },
  { offset: 100, color: "#fd666d" },
];

export const tools = [
  {
    name: "Pointer",
    label: "Select",
    icon: "svguse:icons.svg#cursor|0 0 320 512",
  },
  {
    name: "Box",
    label: "Box",
    icon: "square",
    settings: {
      bgColor: {
        value: "#659dc5",
        type: "color",
        label: "Background Color",
        id: 1,
      },
    },
  },
  {
    name: "Text",
    label: "Text",
    icon: "title",
    settings: {
      textColor: {
        value: "black",
        type: "color",
        label: "Text Color",
        id: 2,
      },
      textAlign: {
        value: "left",
        type: "textAlign",
        id: 3,
      },
      text: {
        value: "Text",
        type: "text",
        label: "Text",
        id: 1,
      },
    },
  },
  {
    name: "Duct",
    label: "Duct",
    icon: "svguse:icons.svg#duct|0 0 226 75",
  },
  {
    name: "Fan",
    label: "Fan",
    icon: "svguse:icons.svg#fan",
    settings: {
      active: {
        value: false,
        type: "boolean",
        label: "Active",
        id: 1,
      },
      inAlarm: {
        value: false,
        type: "boolean",
        label: "In alarm",
        id: 2,
      },
    },
  },
  {
    name: "CoolingCoil",
    label: "Cooling Coil",
    icon: "svguse:icons.svg#cooling-coil|0 0 20 29",
    settings: {
      active: {
        value: false,
        type: "boolean",
        label: "Active",
        id: 1,
      },
      inAlarm: {
        value: false,
        type: "boolean",
        label: "In alarm",
        id: 2,
      },
    },
  },
  {
    name: "HeatingCoil",
    label: "Heating Coil",
    icon: "svguse:icons.svg#heating-coil|0 0 19.526541 28.758413",
    settings: {
      active: {
        value: false,
        type: "boolean",
        label: "Active",
        id: 1,
      },
      inAlarm: {
        value: false,
        type: "boolean",
        label: "In alarm",
        id: 2,
      },
    },
  },
  {
    name: "Filter",
    label: "Filter",
    icon: "svguse:icons.svg#filter|0 0 13 29",
  },
  {
    name: "Humidifier",
    label: "Humidifier",
    icon: "svguse:icons.svg#humidifier|0 0 18 25",
    settings: {
      active: {
        value: false,
        type: "boolean",
        label: "Active",
        id: 1,
      },
      inAlarm: {
        value: false,
        type: "boolean",
        label: "In alarm",
        id: 2,
      },
    },
  },
  {
    name: "Damper",
    label: "Damper",
    icon: "svguse:icons.svg#damper|0 0 13.431694 28.950886",
    settings: {
      inAlarm: {
        value: false,
        type: "boolean",
        label: "In alarm",
        id: 1,
      },
    },
  },
  {
    name: "Temperature",
    label: "Temperature",
    icon: "svguse:icons.svg#temperature|0 0 10.423067 22.852614",
  },
  {
    name: "Gauge",
    label: "Gauge",
    icon: "speed",
    settings: {
      min: {
        value: 0,
        type: "number",
        id: 1,
      },
      max: {
        value: 100,
        type: "number",
        id: 2,
      },
      ticks: {
        value: 10,
        type: "number",
        id: 3,
      },
      minorTicks: {
        value: 5,
        type: "number",
        id: 4,
      },
      thickness: {
        value: 30,
        type: "number",
        id: 5,
      },
      colors: {
        value: gaugeDefautColors,
        type: "colors",
        id: 6,
      },
      titleColor: {
        value: "#000000",
        type: "titleColor",
        id: 7,
      },
    },
  },
  {
    name: "Dial",
    label: "Dial",
    icon: "horizontal_split",
    settings: {
      min: {
        value: 0,
        type: "number",
        id: 1,
      },
      max: {
        value: 100,
        type: "number",
        id: 2,
      },
      ticks: {
        value: 5,
        type: "number",
        id: 3,
      },
      minorTicks: {
        value: 5,
        type: "number",
        id: 4,
      },
      colors: {
        value: gaugeDefautColors,
        type: "colors",
        id: 5,
      },
      titleColor: {
        value: "#000000",
        type: "titleColor",
        id: 6,
      },
      textColor: {
        value: "#000000",
        type: "color",
        label: "Text Color",
        id: 7,
      },
    },
  },
  {
    name: "Value",
    label: "Value",
    icon: "123",
    settings: {
      bgColor: {
        value: "#0065a3",
        type: "color",
        label: "Background Color",
        id: 1,
      },
      textColor: {
        value: "#ffffff",
        type: "color",
        label: "Text Color",
        id: 3,
      },
      titleColor: {
        value: "#000000",
        type: "titleColor",
        id: 2,
      },
      justifyContent: {
        value: "flex-start",
        type: "justifyContent",
        id: 4,
      },
    },
  },
  {
    name: "Icon",
    label: "Icon with title",
    icon: "fa-solid fa-icons",
    settings: {
      justifyContent: {
        value: "flex-start",
        type: "justifyContent",
        id: 2,
      },
      active: {
        value: false,
        type: "boolean",
        label: "On/Off",
        id: 1,
      },
      offColor: {
        value: "#940303",
        type: "color",
        label: "Off Color",
        id: 3,
      },
      onColor: {
        value: "#0d87d9",
        type: "color",
        label: "On Color",
        id: 4,
      },
      icon: {
        value: "fa-solid fa-camera-retro",
        type: "icon",
        label: "Icon",
        id: 5,
      },
      t3EntryDisplayField: {
        value: "value",
        type: "select",
        id: 6,
      },
    },
  },
];

export const icons = [
  {
    value: "",
    label: "None",
  },
  {
    value: "image",
    label: "Image",
  },
  {
    value: "schedule",
    label: "Schedule",
  },
  {
    value: "power_settings_new",
    label: "Power Button",
  },
  {
    value: "bolt",
    label: "Bolt",
  },
  {
    value: "grid_view",
    label: "Grid",
  },
  {
    value: "question_mark",
    label: "Question mark",
  },
  {
    value: "help",
    label: "Help",
  },
  {
    value: "error",
    label: "Error",
  },
  {
    value: "cancel",
    label: "Cancel",
  },
  {
    value: "check",
    label: "Check",
  },
  {
    value: "check_circle",
    label: "Check circle",
  },
  {
    value: "visibility",
    label: "Visibility",
  },
  {
    value: "fa-solid fa-camera-retro",
    label: "Camera Retro",
  },
  {
    value: "fa-solid fa-laptop-code",
    label: "Code",
  },
  {
    value: "calendar_month",
    label: "Calendar",
  },
];

export const ranges = [
  {
    id: 1,
    label: "Off/On",
    off: "Off",
    on: "On",
    directInvers: null,
  },
  {
    id: 2,
    label: "Close/Open",
    off: "Close",
    on: "Open",
    directInvers: null,
  },
  {
    id: 3,
    label: "Stop/Start",
    off: "Stop",
    on: "Start",
    directInvers: null,
  },
  {
    id: 4,
    label: "Disable/Enable",
    off: "Disable",
    on: "Enable",
    directInvers: null,
  },
  {
    id: 5,
    label: "Normal/Alarm",
    off: "Normal",
    on: "Alarm",
    directInvers: null,
  },
  {
    id: 6,
    label: "Normal/High",
    off: "Normal",
    on: "High",
    directInvers: null,
  },
  {
    id: 7,
    label: "Normal/Low",
    off: "Normal",
    on: "Low",
    directInvers: null,
  },
  {
    id: 8,
    label: "No/Yes",
    off: "No",
    on: "Yes",
    directInvers: null,
  },
  {
    id: 9,
    label: "Cool/Heat",
    off: "Cool",
    on: "Heat",
    directInvers: null,
  },
  {
    id: 10,
    label: "Unoccupy/Occupy",
    off: "Unoccupy",
    on: "Occupy",
    directInvers: null,
  },
  {
    id: 11,
    label: "Low/High",
    on: "Low",
    off: "High",
    directInvers: null,
  },
  {
    id: 12,
    label: "On/Off",
    on: "Off",
    off: "On",
    directInvers: true,
  },
  {
    id: 13,
    label: "Open/Close",
    on: "Close",
    off: "Open",
    directInvers: true,
  },
  {
    id: 14,
    label: "Start/Stop",
    on: "Stop",
    off: "Start",
    directInvers: true,
  },
  {
    id: 15,
    label: "Enable/Disable",
    on: "Disable",
    off: "Enable",
    directInvers: true,
  },
  {
    id: 16,
    label: "Alarm/Normal",
    on: "Normal",
    off: "Alarm",
    directInvers: true,
  },
  {
    id: 17,
    label: "High/Normal",
    on: "Normal",
    off: "High",
    directInvers: true,
  },
  {
    id: 18,
    label: "Low/Normal",
    on: "Normal",
    off: "Low",
    directInvers: true,
  },
  {
    id: 19,
    label: "Yes/No",
    on: "No",
    off: "Yes",
    directInvers: true,
  },
  {
    id: 20,
    label: "Heat/Cool",
    on: "Cool",
    off: "Heat",
    directInvers: true,
  },
  {
    id: 21,
    label: "Occupy/Unoccupy",
    on: "Unoccupy",
    off: "Occupy",
    directInvers: true,
  },
  {
    id: 22,
    label: "High/Low",
    on: "Low",
    off: "High",
    directInvers: true,
  },
  {
    analog: true,
    id: 0,
    unit: "",
    label: "Unused",
  },
  {
    analog: true,
    id: 1,
    unit: "Deg.C",
    label: "Y3K -40 to 150",
  },
  {
    analog: true,
    id: 2,
    unit: "Deg.F",
    label: "Y3K -40 to 300",
  },
  {
    analog: true,
    id: 3,
    unit: "Deg.C",
    label: "10K Type2",
  },
  {
    analog: true,
    id: 4,
    unit: "Deg.F",
    label: "10K Type2",
  },
  {
    analog: true,
    id: 5,
    unit: "Deg.C",
    label: "G3K -40 to 120",
  },
  {
    analog: true,
    id: 6,
    unit: "Deg.F",
    label: "G3K -40 to 250",
  },
  {
    analog: true,
    id: 7,
    unit: "Deg.C",
    label: "10K Type3",
  },
  {
    analog: true,
    id: 8,
    unit: "Deg.F",
    label: "10K Type3",
  },
  {
    analog: true,
    id: 9,
    unit: "Deg.C",
    label: "PT 1K -200 to 300",
  },
  {
    analog: true,
    id: 10,
    unit: "Deg.F",
    label: "PT 1K -200 to 570",
  },
  {
    analog: true,
    id: 11,
    unit: "Volts",
    label: "0.0 to 5.0",
  },
  {
    analog: true,
    id: 12,
    unit: "Amps",
    label: "0.0 to 100",
  },
  {
    analog: true,
    id: 13,
    unit: "ma",
    label: "4 to 20",
  },
  {
    analog: true,
    id: 14,
    unit: "psi",
    label: "4 to 20",
  },
  {
    analog: true,
    id: 15,
    unit: "counts",
    label: "Pulse Count (Slow 1Hz)",
  },
  {
    analog: true,
    id: 16,
    unit: "%",
    label: "0 to 100",
  },
  {
    analog: true,
    id: 17,
    unit: "%",
    label: "0 to 100",
  },
  {
    analog: true,
    id: 18,
    unit: "%",
    label: "0 to 100",
  },
  {
    analog: true,
    id: 19,
    unit: "Volts",
    label: "0.0 to 10.0",
  },
  {
    analog: true,
    id: 20,
    unit: "",
    label: "Table 1",
  },
  {
    analog: true,
    id: 21,
    unit: "",
    label: "Table 2",
  },
  {
    analog: true,
    id: 22,
    unit: "",
    label: "Table 3",
  },
  {
    analog: true,
    id: 23,
    unit: "",
    label: "Table 4",
  },
  {
    analog: true,
    id: 24,
    unit: "",
    label: "Table 5",
  },
  {
    analog: true,
    id: 25,
    unit: "counts",
    label: "Pulse Count (Fast 100Hz)",
  },
  {
    analog: true,
    id: 26,
    unit: "Hz",
    label: "Frequency",
  },
  {
    analog: true,
    id: 27,
    unit: "%",
    label: "Humidty %",
  },
  {
    analog: true,
    id: 28,
    unit: "PPM",
    label: "CO2  PPM",
  },
  {
    analog: true,
    id: 29,
    unit: "RPM",
    label: "Revolutions Per Minute",
  },
  {
    analog: true,
    id: 30,
    unit: "PPB",
    label: "TVOC PPB",
  },
  {
    analog: true,
    id: 31,
    unit: "ug/m3",
    label: "ug/m3",
  },
  {
    analog: true,
    id: 32,
    unit: "#/cm3",
    label: "#/cm3",
  },
  {
    analog: true,
    id: 33,
    unit: "dB",
    label: "dB",
  },
  {
    analog: true,
    id: 34,
    unit: "Lux",
    label: "Lux",
  },
  {
    analog: true,
    id: 35,
    unit: "",
    label: "",
  },
  {
    analog: true,
    id: 36,
    unit: "",
    label: "",
  },
  {
    analog: true,
    id: 37,
    unit: "",
    label: "",
  },
  {
    analog: true,
    id: 38,
    unit: "",
    label: "",
  },
  {
    analog: true,
    id: 39,
    unit: "",
    label: "",
  },
];
