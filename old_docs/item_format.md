# 物品数据结构文档

## 概述

骑砍战团的物品数据存储在 `item_kinds1.txt` 文件中，采用特定的二进制格式。每个物品包含基础属性、模型信息、战斗属性和触发器数据。

## 文件结构

### 文件头

```
版本信息 (3个字段)
物品总数量 (1个整数)
```

### 物品数据结构

#### 基础信息

```vb
Public Type Type_Item
    ID As Long                  ' 物品ID (自动生成)
    dbName As String           ' 数据库名称 (唯一标识符)
    disname As String          ' 显示名称
    texname As String          ' 贴图名称
    csvName As String          ' CSV文件中的名称
    csvName_pl As String       ' 复数形式名称
End Type
```

#### 模型信息

```vb
Type Type_Item_Model
    nmdl As Long               ' 模型数量
    mdlname() As String        ' 模型文件名数组
    mdl_b() As String          ' 模型绑定信息数组
End Type
```

#### 物品属性

```vb
Type Type_Item_Properties
    itmType As String          ' 物品类型标识
    Action As String           ' 动作标识
    price As Long              ' 价格
    Prefix As String           ' 前缀标识
    weight As String           ' 重量
    abundance As Long          ' 丰富度
    
    ' 护甲属性
    head_armor As Long         ' 头部护甲值
    body_armor As Long         ' 身体护甲值
    leg_armor As Long          ' 腿部护甲值
    
    ' 武器属性
    difficulty As Long         ' 使用难度
    hit_points As Long         ' 耐久度
    speed_rating As Long       ' 速度等级
    missile_speed As Long      ' 投射物速度
    weapon_length As Long      ' 武器长度
    max_ammo As Long           ' 最大弹药
    thrust_damage As Long      ' 刺击伤害
    swing_damage As Long       ' 挥砍伤害
End Type
```

#### 派系限制

```vb
Type Type_Item_Faction
    FactionCount As Long       ' 限制派系数量
    Faction() As Type_Faction_Ref  ' 派系引用数组
End Type

Type Type_Faction_Ref
    ID As String               ' 派系ID
End Type
```

#### 触发器系统

```vb
Type Type_Item_Triggers
    TriggerCount As Long       ' 触发器数量
    Trigger() As Type_Trigger  ' 触发器数组
End Type

Type Type_Trigger
    tiOn As Long               ' 触发条件
    ActNum As Long             ' 动作数量
    tiAct() As Type_Action     ' 动作数组
End Type

Type Type_Action
    Op As String               ' 操作码
    ParaNum As Long            ' 参数数量
    Para() As String           ' 参数数组
End Type
```

## 物品类型常量

### 基础类型

```vb
Public Const itp_type_horse = &H1           ' 马匹
Public Const itp_type_one_handed_wpn = &H2  ' 单手武器
Public Const itp_type_two_handed_wpn = &H3  ' 双手武器
Public Const itp_type_polearm = &H4         ' 长柄武器
Public Const itp_type_arrows = &H5          ' 箭矢
Public Const itp_type_bolts = &H6           ' 弩箭
Public Const itp_type_shield = &H7          ' 盾牌
Public Const itp_type_bow = &H8             ' 弓
Public Const itp_type_crossbow = &H9        ' 弩
Public Const itp_type_thrown = &HA          ' 投掷武器
Public Const itp_type_goods = &HB           ' 商品
Public Const itp_type_head_armor = &HC      ' 头盔
Public Const itp_type_body_armor = &HD      ' 身甲
Public Const itp_type_foot_armor = &HE      ' 靴子
Public Const itp_type_hand_armor = &HF      ' 手套
Public Const itp_type_pistol = &H10         ' 手枪
Public Const itp_type_musket = &H11         ' 火枪
Public Const itp_type_bullets = &H12        ' 子弹
Public Const itp_type_animal = &H13         ' 动物
Public Const itp_type_book = &H14           ' 书籍
```

### 物品标志位

```vb
' 装备位置
Public Const itp_force_attach_left_hand = "0000000000000100"
Public Const itp_force_attach_right_hand = "0000000000000200"
Public Const itp_force_attach_left_forearm = "0000000000000300"

' 物品属性
Public Const itp_unique = 12                ' 唯一物品
Public Const itp_always_loot = 13          ' 总是掉落
Public Const itp_no_parry = 14             ' 无法格挡
Public Const itp_default_ammo = 15         ' 默认弹药
Public Const itp_merchandise = 16          ' 商品
Public Const itp_wooden_attack = 17        ' 木质攻击
Public Const itp_wooden_parry = 18         ' 木质格挡
Public Const itp_food = 19                 ' 食物

' 武器特性
Public Const itp_cant_reload_on_horseback = 20  ' 马上无法装填
Public Const itp_two_handed = 21               ' 双手武器
Public Const itp_primary = 22                  ' 主武器
Public Const itp_secondary = 23                ' 副武器
Public Const itp_covers_legs = 24              ' 覆盖腿部
Public Const itp_consumable = 25               ' 消耗品
Public Const itp_bonus_against_shield = 26     ' 对盾牌加成
Public Const itp_penalty_with_shield = 27      ' 持盾惩罚
Public Const itp_cant_use_on_horseback = 28    ' 马上无法使用
Public Const itp_civilian = 29                 ' 平民装备
Public Const itp_covers_head = 31              ' 覆盖头部
Public Const itp_couchable = 31                ' 可平端
Public Const itp_crush_through = 32            ' 破甲
Public Const itp_knock_back = 33               ' 击退
Public Const itp_unbalanced = 35               ' 不平衡
```

## 战斗能力标志

### 攻击方式

```vb
' 单手攻击
Public Const itcf_Thrust_onehanded = "0000000000000001"      ' 单手刺击
Public Const itcf_Overswing_onehanded = "0000000000000002"   ' 单手上劈
Public Const itcf_Slashright_onehanded = "0000000000000004"  ' 单手右砍
Public Const itcf_Slashleft_onehanded = "0000000000000008"   ' 单手左砍

' 双手攻击
Public Const itcf_Thrust_twohanded = "0000000000000010"      ' 双手刺击
Public Const itcf_Overswing_twohanded = "0000000000000020"   ' 双手上劈
Public Const itcf_Slashright_twohanded = "0000000000000040"  ' 双手右砍
Public Const itcf_Slashleft_twohanded = "0000000000000080"   ' 双手左砍

' 长柄武器攻击
Public Const itcf_Thrust_polearm = "0000000000000100"        ' 长柄刺击
Public Const itcf_Overswing_polearm = "0000000000000200"     ' 长柄上劈
Public Const itcf_Slashright_polearm = "0000000000000400"    ' 长柄右砍
Public Const itcf_Slashleft_polearm = "0000000000000800"     ' 长柄左砍

' 远程攻击
Public Const itcf_Shoot_bow = "0000000000001000"             ' 弓射击
Public Const itcf_Shoot_javelin = "0000000000002000"         ' 标枪投掷
Public Const itcf_Shoot_crossbow = "0000000000004000"        ' 弩射击
```

### 马战能力

```vb
Public Const itcf_Horseback_thrust_onehanded = "0000000000100000"        ' 马上单手刺击
Public Const itcf_Horseback_overswing_right_onehanded = "0000000000200000" ' 马上单手右劈
Public Const itcf_Horseback_overswing_left_onehanded = "0000000000400000"  ' 马上单手左劈
Public Const itcf_Horseback_slashright_onehanded = "0000000000800000"      ' 马上单手右砍
Public Const itcf_Horseback_slashleft_onehanded = "0000000001000000"       ' 马上单手左砍
```

## 数据解析流程

### 1. 文件读取

```vb
Sub LoadItemFile(FilePath As String)
    ' 打开文件
    MaxPointer = FileLen(tmpFileName)
    lngHandle = FreeFile()
    Open tmpFileName For Random Access Read As lngHandle Len = 1
    
    ' 读取版本信息
    For n = 0 To 2
        ItmVersionInform(n) = GetWord()
    Next n
    
    ' 读取物品数量
    N_Item = Val(GetWord())
    ReDim itm(N_Item - 1)
End Sub
```

### 2. 物品数据读取

```vb
For n = 0 To N_Item - 1
    With itm(n)
        .ID = n
        .dbName = GetWord()
        .disname = GetWord()
        .texname = GetWord()
        .nmdl = Val(GetWord())
        
        ' 读取模型信息
        If .nmdl > 0 Then
            ReDim .mdlname(1 To .nmdl)
            ReDim .mdl_b(1 To .nmdl)
            For m = 1 To .nmdl
                .mdlname(m) = GetWord()
                .mdl_b(m) = GetWord()
            Next m
        End If
        
        ' 读取属性
        .itmType = GetWord()
        .Action = GetWord()
        .price = Val(GetWord())
        .Prefix = GetWord()
        .weight = GetWord()
        .abundance = Val(GetWord())
        
        ' 读取护甲值
        .head_armor = Val(GetWord())
        .body_armor = Val(GetWord())
        .leg_armor = Val(GetWord())
        
        ' 读取武器属性
        .difficulty = Val(GetWord())
        .hit_points = Val(GetWord())
        .speed_rating = Val(GetWord())
        .missile_speed = Val(GetWord())
        .weapon_length = Val(GetWord())
        .max_ammo = Val(GetWord())
        .thrust_damage = Val(GetWord())
        .swing_damage = Val(GetWord())
        
        ' 读取派系限制
        .FactionCount = Val(GetWord())
        If .FactionCount > 0 Then
            ReDim .Faction(1 To .FactionCount)
            For m = 1 To .FactionCount
                .Faction(m).ID = GetWord()
            Next m
        End If
        
        ' 读取触发器数据
        .TriggerCount = Val(GetWord())
        If .TriggerCount > 0 Then
            ReDim .Trigger(1 To .TriggerCount)
            For i = 1 To .TriggerCount
                .Trigger(i).tiOn = Val(GetWord())
                .Trigger(i).ActNum = Val(GetWord())
                If .Trigger(i).ActNum > 0 Then
                    ReDim .Trigger(i).tiAct(1 To .Trigger(i).ActNum)
                    For m = 1 To .Trigger(i).ActNum
                        .Trigger(i).tiAct(m).Op = GetWord()
                        .Trigger(i).tiAct(m).ParaNum = Val(GetWord())
                        If .Trigger(i).tiAct(m).ParaNum > 0 Then
                            ReDim .Trigger(i).tiAct(m).Para(1 To .Trigger(i).tiAct(m).ParaNum)
                            For H = 1 To .Trigger(i).tiAct(m).ParaNum
                                .Trigger(i).tiAct(m).Para(H) = GetWord()
                            Next H
                        End If
                    Next m
                End If
            Next i
        End If
    End With
Next n
```

## 数据验证规则

### 必填字段

- `dbName`: 必须唯一，不能为空
- `disname`: 显示名称，不能为空
- `itmType`: 必须是有效的物品类型

### 数值范围

- `price`: >= 0
- `weight`: >= 0
- `abundance`: 0-100
- `difficulty`: 0-20
- `hit_points`: >= 0
- 伤害值: >= 0

### 引用完整性

- 模型文件必须存在于 `Resource/` 目录
- 贴图文件必须存在于 `Textures/` 目录
- 派系ID必须在 `factions.txt` 中定义

## 常见问题

### Q: 如何添加新的物品类型？

A: 需要在常量定义中添加新的类型标识，并更新相关的UI和验证逻辑。

### Q: 触发器数据如何工作？

A: 触发器定义了物品在特定条件下的行为，如使用时的效果、装备时的加成等。

### Q: 为什么有些字段是字符串而不是数值？

A: 为了保持与原版数据格式的兼容性，某些数值字段以字符串形式存储。

---

*本文档基于魔球编辑器源码分析，详细描述了骑砍战团物品数据的存储格式和解析方法。*
