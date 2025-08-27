# 文件格式规范文档

## 概述

骑砍战团使用多种文件格式存储游戏数据，主要包括二进制数据文件(.txt)和多语言支持文件(.csv)。魔球编辑器需要解析和生成这些格式的文件。

## 通用文件结构

### 二进制数据文件格式

所有主要数据文件都遵循相似的结构：

```
文件头:
  版本信息 (3个32位整数)
  数据项数量 (32位整数)

数据项列表:
  数据项1
  数据项2
  ...
  数据项N
```

### 字符串编码

#### GetWord() 函数实现

```vb
Function GetWord() As String
    Dim strTemp As String
    Dim bytTemp As Byte
    
    ' 跳过空白字符
    Do While Pointer <= MaxPointer
        Get lngHandle, Pointer, bytTemp
        If bytTemp <> 32 And bytTemp <> 9 And bytTemp <> 10 And bytTemp <> 13 Then
            Exit Do
        End If
        Pointer = Pointer + 1
    Loop
    
    ' 读取字符串
    strTemp = ""
    Do While Pointer <= MaxPointer
        Get lngHandle, Pointer, bytTemp
        If bytTemp = 32 Or bytTemp = 9 Or bytTemp = 10 Or bytTemp = 13 Then
            Exit Do
        End If
        strTemp = strTemp + Chr(bytTemp)
        Pointer = Pointer + 1
    Loop
    
    GetWord = strTemp
End Function
```

## 具体文件格式

### 1. item_kinds1.txt (物品数据)

#### 文件结构

```
版本信息: [version1] [version2] [version3]
物品数量: [item_count]

对于每个物品:
  数据库名称: [db_name]
  显示名称: [display_name]  
  贴图名称: [texture_name]
  模型数量: [model_count]
  
  对于每个模型:
    模型文件名: [model_file]
    模型绑定: [model_binding]
  
  物品类型: [item_type]
  动作类型: [action_type]
  价格: [price]
  前缀: [prefix]
  重量: [weight]
  丰富度: [abundance]
  头部护甲: [head_armor]
  身体护甲: [body_armor]
  腿部护甲: [leg_armor]
  使用难度: [difficulty]
  耐久度: [hit_points]
  速度等级: [speed_rating]
  投射物速度: [missile_speed]
  武器长度: [weapon_length]
  最大弹药: [max_ammo]
  刺击伤害: [thrust_damage]
  挥砍伤害: [swing_damage]
  
  派系数量: [faction_count]
  对于每个派系:
    派系ID: [faction_id]
  
  触发器数量: [trigger_count]
  对于每个触发器:
    触发条件: [trigger_condition]
    动作数量: [action_count]
    对于每个动作:
      操作码: [operation_code]
      参数数量: [param_count]
      对于每个参数:
        参数值: [parameter_value]
```

#### 示例数据

```
1158 0 0
685
no_item No_Item none 0 itp_type_goods 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
crude_throwing_daggers Crude_Throwing_Daggers throwing_dagger_a 1 throwing_dagger_a 0 itp_type_thrown itc_throwing 4 0 1.5 90 0 0 0 7 85 0 85 0 15 0 0 0
```

### 2. troops.txt (兵种数据)

#### 文件结构

```
版本信息: [version1] [version2] [version3]
兵种数量: [troop_count]

对于每个兵种:
  数据库名称: [db_name]
  显示名称: [display_name]
  场景设置: [scene]
  保留字段: [reserved]
  网格模型: [mesh]
  派系: [faction]
  升级标志: [upgrade_flags]
  是否英雄: [is_hero]
  等级: [level]
  经验值: [exp]
  
  属性点:
    力量: [strength]
    敏捷: [agility]
    智力: [intelligence]
    魅力: [charisma]
  
  武器熟练度:
    单手武器: [one_handed]
    双手武器: [two_handed]
    长柄武器: [polearm]
    弓箭: [archery]
    弩: [crossbow]
    投掷: [throwing]
    火器: [firearms]
  
  技能点: [skill1] [skill2] ... [skill42]
  
  面部特征:
    面部特征码1: [face_key_1]
    面部特征码2: [face_key_2]
    头发颜色: [hair_color]
    头发材质: [hair_texture]
    面部材质: [face_texture]
    声音: [voice]
    身体颜色: [body_color]
    面部保留: [reserved_face]
  
  装备数量: [equipment_count]
  对于每个装备:
    物品ID: [item_id]
    物品修正: [item_modifier]
```

### 3. parties.txt (队伍数据)

#### 文件结构

```
版本信息: [version1] [version2] [version3]
队伍数量: [party_count]

对于每个队伍:
  数据库名称: [db_name]
  显示名称: [display_name]
  标志: [flags]
  菜单: [menu]
  派系: [faction]
  个性: [personality]
  模板: [template]
  
  兵种栈数量: [stack_count]
  对于每个兵种栈:
    兵种ID: [troop_id]
    数量: [count]
    经验: [experience]
    标志: [flags]
```

### 4. factions.txt (派系数据)

#### 文件结构

```
版本信息: [version1] [version2] [version3]
派系数量: [faction_count]

对于每个派系:
  数据库名称: [db_name]
  显示名称: [display_name]
  标志: [flags]
  一致性: [coherence]
  颜色: [color]
  
  关系数量: [relation_count]
  对于每个关系:
    目标派系: [target_faction]
    关系值: [relation_value]
```

### 5. scenes.txt (场景数据)

#### 文件结构

```
版本信息: [version1] [version2] [version3]
场景数量: [scene_count]

对于每个场景:
  数据库名称: [db_name]
  标志: [flags]
  网格名称: [mesh_name]
  身体名称: [body_name]
  外部空间: [outer_mesh]
  
  通道数量: [passage_count]
  对于每个通道:
    通道标志: [passage_flags]
    场景: [scene]
    位置1: [pos1]
    位置2: [pos2]
```

## CSV文件格式

### 多语言支持文件

CSV文件用于存储不同语言的文本内容：

#### item_kinds.csv

```csv
itm_no_item,No Item,无物品
itm_crude_throwing_daggers,Crude Throwing Daggers,粗制飞刀
itm_throwing_daggers,Throwing Daggers,飞刀
```

#### troops.csv

```csv
trp_player,Player,玩家
trp_multiplayer_profile_troop_male,Multiplayer Profile Troop Male,多人游戏男性角色
trp_multiplayer_profile_troop_female,Multiplayer Profile Troop Female,多人游戏女性角色
```

### CSV解析规则

1. 第一列：数据库名称（与.txt文件中的db_name对应）
2. 第二列：英文名称
3. 第三列：本地化名称
4. 支持UTF-8编码
5. 使用逗号分隔
6. 支持引号包围的字符串

## 数据类型定义

### 基础数据类型

#### 整数类型

- `Byte`: 8位无符号整数 (0-255)
- `Integer`: 16位有符号整数 (-32768 to 32767)
- `Long`: 32位有符号整数 (-2147483648 to 2147483647)
- `Integer64b`: 自定义64位整数类型

#### 字符串类型

- `String`: 变长字符串，UTF-8编码
- 空字符串表示为 "0" 或空白

#### 标志位类型

- 使用64位整数存储多个布尔标志
- 每一位代表一个特定的属性或状态

### 复合数据类型

#### 位置类型

```vb
Type Type_Position
    x As Double
    y As Double
    z As Double
    rotation_x As Double
    rotation_y As Double
    rotation_z As Double
    rotation_w As Double
End Type
```

#### 颜色类型

```vb
Type Type_Color
    r As Long  ' 红色分量 (0-255)
    g As Long  ' 绿色分量 (0-255)
    b As Long  ' 蓝色分量 (0-255)
    a As Long  ' 透明度 (0-255)
End Type
```

## 文件I/O操作

### 读取操作

#### 打开文件

```vb
Sub OpenDataFile(FilePath As String)
    If FileLen(FilePath) = 0 Then
        MsgBox "文件不存在或为空: " & FilePath
        Exit Sub
    End If
    
    MaxPointer = FileLen(FilePath)
    lngHandle = FreeFile()
    Open FilePath For Random Access Read As lngHandle Len = 1
    Pointer = 1
End Sub
```

#### 读取版本信息

```vb
Sub ReadVersionInfo(VersionArray() As Long)
    For i = 0 To 2
        VersionArray(i) = Val(GetWord())
    Next i
End Sub
```

#### 读取数据项数量

```vb
Function ReadItemCount() As Long
    ReadItemCount = Val(GetWord())
End Function
```

### 写入操作

#### 保存文件

```vb
Sub SaveDataFile(FilePath As String, Data As Variant)
    Dim FileHandle As Integer
    FileHandle = FreeFile()
    
    Open FilePath For Output As FileHandle
    
    ' 写入版本信息
    Print #FileHandle, "1158 0 0"
    
    ' 写入数据项数量
    Print #FileHandle, UBound(Data) + 1
    
    ' 写入数据项
    For i = 0 To UBound(Data)
        WriteDataItem FileHandle, Data(i)
    Next i
    
    Close FileHandle
End Sub
```

## 数据验证

### 文件完整性检查

#### 版本兼容性

```vb
Function CheckVersionCompatibility(Version() As Long) As Boolean
    ' 检查版本是否兼容
    If Version(0) >= 1158 Then
        CheckVersionCompatibility = True
    Else
        CheckVersionCompatibility = False
        MsgBox "不支持的文件版本: " & Version(0)
    End If
End Function
```

#### 数据范围验证

```vb
Function ValidateDataRange(Value As Long, MinVal As Long, MaxVal As Long) As Boolean
    ValidateDataRange = (Value >= MinVal And Value <= MaxVal)
End Function
```

#### 引用完整性检查

```vb
Function ValidateReference(RefID As String, ValidIDs() As String) As Boolean
    For i = 0 To UBound(ValidIDs)
        If RefID = ValidIDs(i) Then
            ValidateReference = True
            Exit Function
        End If
    Next i
    ValidateReference = False
End Function
```

## 错误处理

### 常见错误类型

#### 文件错误

- 文件不存在
- 文件损坏
- 权限不足
- 磁盘空间不足

#### 数据错误

- 格式不正确
- 数据超出范围
- 引用无效
- 编码错误

#### 处理策略

```vb
Sub HandleFileError(ErrorType As String, ErrorMsg As String)
    Select Case ErrorType
        Case "FILE_NOT_FOUND"
            MsgBox "文件未找到: " & ErrorMsg
        Case "INVALID_FORMAT"
            MsgBox "文件格式无效: " & ErrorMsg
        Case "DATA_CORRUPTION"
            MsgBox "数据损坏: " & ErrorMsg
        Case Else
            MsgBox "未知错误: " & ErrorMsg
    End Select
End Sub
```

## 性能优化

### 内存管理

- 使用动态数组避免内存浪费
- 及时释放不需要的对象
- 批量处理减少I/O操作

### 读取优化

- 缓存频繁访问的数据
- 使用索引加速查找
- 延迟加载大型数据

### 写入优化

- 批量写入减少磁盘操作
- 使用缓冲区提高效率
- 压缩重复数据

## 扩展性设计

### 版本兼容性

- 向后兼容旧版本文件
- 支持新字段的添加
- 优雅处理未知数据

### 模块化设计

- 独立的解析器模块
- 可插拔的验证器
- 灵活的输出格式

---

*本文档详细描述了骑砍战团各种数据文件的格式规范和处理方法，为编辑器开发提供技术参考。*
