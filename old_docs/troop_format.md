# 兵种数据结构文档

## 概述

骑砍战团的兵种数据存储在 `troops.txt` 文件中，定义了游戏中所有角色的属性、装备、技能和AI行为。兵种系统是游戏的核心组成部分，包括玩家角色、NPC、士兵等。

## 文件结构

### 文件头

```
版本信息 (3个字段)
兵种总数量 (1个整数)
```

### 兵种数据结构

#### 基础信息

```vb
Public Type Type_Troop
    ID As Long                  ' 兵种ID (自动生成)
    dbName As String           ' 数据库名称 (唯一标识符)
    disname As String          ' 显示名称
    csvName As String          ' CSV文件中的名称
    csvName_pl As String       ' 复数形式名称
    
    ' 外观设置
    scene As String            ' 场景设置
    reserved As String         ' 保留字段
    mesh As String             ' 网格模型
    
    ' 基础属性
    faction As Long            ' 所属派系
    upgrade_flags As String    ' 升级标志
    is_hero As Long            ' 是否为英雄
    
    ' 经验和等级
    level As Long              ' 等级
    exp As Long                ' 经验值
End Type
```

#### 属性点分配

```vb
Type Type_Troop_Attributes
    strength As Long           ' 力量
    agility As Long            ' 敏捷
    intelligence As Long       ' 智力
    charisma As Long           ' 魅力
End Type
```

#### 武器熟练度

```vb
Type Type_Troop_Proficiencies
    one_handed As Long         ' 单手武器
    two_handed As Long         ' 双手武器
    polearm As Long            ' 长柄武器
    archery As Long            ' 弓箭
    crossbow As Long           ' 弩
    throwing As Long           ' 投掷
    firearms As Long           ' 火器
End Type
```

#### 技能系统

```vb
Type Type_Troop_Skills
    ' 个人技能
    ironflesh As Long          ' 铁骨
    power_draw As Long         ' 强弓
    power_throw As Long        ' 强掷
    power_strike As Long       ' 强击
    athletics As Long          ' 跑动
    riding As Long             ' 骑术
    horse_archery As Long      ' 骑射
    shield As Long             ' 盾防
    weapon_master As Long      ' 武器掌握
    
    ' 领导技能
    leadership As Long         ' 领导力
    prisoner_management As Long ' 俘虏管理
    first_aid As Long          ' 急救
    surgery As Long            ' 外科手术
    wound_treatment As Long    ' 伤口处理
    inventory_management As Long ' 物品管理
    spotting As Long           ' 侦察
    pathfinding As Long        ' 向导
    tactics As Long            ' 战术
    tracking As Long           ' 追踪
    trainer As Long            ' 训练
    engineer As Long           ' 工程学
    trade As Long              ' 交易
    persuasion As Long         ' 说服
    looting As Long            ' 掠夺
End Type
```

#### 装备配置

```vb
Type Type_Troop_Equipment
    item_count As Long         ' 装备数量
    items() As Type_Equipment_Item  ' 装备数组
End Type

Type Type_Equipment_Item
    item_id As String          ' 物品ID
    imod As Long               ' 物品修正
End Type
```

#### 面部特征

```vb
Type Type_Troop_Face
    face_key_1 As String       ' 面部特征码1
    face_key_2 As String       ' 面部特征码2
    hair_color As Long         ' 头发颜色
    hair_texture As Long       ' 头发材质
    face_texture As Long       ' 面部材质
    voice As Long              ' 声音
    body_color As Long         ' 身体颜色
    reserved_face As String    ' 面部保留字段
End Type
```

## 兵种标志常量

### 基础标志

```vb
Public Const tf_hero = 0                    ' 英雄
Public Const tf_randomize_face = 1          ' 随机面部
Public Const tf_guarantee_boots = 2         ' 保证靴子
Public Const tf_guarantee_armor = 3         ' 保证护甲
Public Const tf_guarantee_helmet = 4        ' 保证头盔
Public Const tf_guarantee_horse = 5         ' 保证马匹
Public Const tf_guarantee_shield = 6        ' 保证盾牌
Public Const tf_guarantee_weapon = 7        ' 保证武器
Public Const tf_guarantee_ranged = 8        ' 保证远程武器
Public Const tf_is_merchant = 9             ' 商人
Public Const tf_guarantee_gloves = 10       ' 保证手套
Public Const tf_guarantee_all = 11          ' 保证所有装备
Public Const tf_unmoveable_in_party_window = 12  ' 队伍窗口不可移动
```

### 升级路径标志

```vb
Public Const tf_guarantee_all_wo_ranged = 13    ' 保证除远程外所有装备
Public Const tf_guarantee_armor_wo_gloves = 14  ' 保证除手套外护甲
Public Const tf_guarantee_boots_wo_ranged = 15  ' 保证靴子但不保证远程
Public Const tf_guarantee_helmet_wo_ranged = 16 ' 保证头盔但不保证远程
```

## AI行为常量

### 基础AI行为

```vb
Public Const ai_bhvr_hold = 0               ' 保持位置
Public Const ai_bhvr_follow = 1             ' 跟随
Public Const ai_bhvr_charge = 2             ' 冲锋
Public Const ai_bhvr_stand_ground = 3       ' 坚守阵地
Public Const ai_bhvr_retreat = 4            ' 撤退
Public Const ai_bhvr_advance = 5            ' 前进
Public Const ai_bhvr_fall_back = 6          ' 后退
Public Const ai_bhvr_stand_closer = 7       ' 靠近
Public Const ai_bhvr_spread_out = 8         ' 散开
Public Const ai_bhvr_use_blunt_weapons = 9  ' 使用钝器
Public Const ai_bhvr_avoid_ranged_weapons = 10  ' 避免远程武器
Public Const ai_bhvr_mounted = 11           ' 骑兵模式
```

## 数据解析流程

### 1. 文件读取

```vb
Sub LoadTroopFile(FilePath As String)
    ' 打开文件
    MaxPointer = FileLen(tmpFileName)
    lngHandle = FreeFile()
    Open tmpFileName For Random Access Read As lngHandle Len = 1
    
    ' 读取版本信息
    For n = 0 To 2
        TrpVersionInform(n) = GetWord()
    Next n
    
    ' 读取兵种数量
    N_Troop = Val(GetWord())
    ReDim trp(N_Troop - 1)
End Sub
```

### 2. 兵种数据读取

```vb
For n = 0 To N_Troop - 1
    With trp(n)
        .ID = n
        .dbName = GetWord()
        .disname = GetWord()
        .csvName = .disname
        .csvName_pl = .disname
        
        ' 读取基础信息
        .scene = GetWord()
        .reserved = GetWord()
        .mesh = GetWord()
        .faction = Val(GetWord())
        .upgrade_flags = GetWord()
        .is_hero = Val(GetWord())
        
        ' 读取等级和经验
        .level = Val(GetWord())
        .exp = Val(GetWord())
        
        ' 读取属性点
        .strength = Val(GetWord())
        .agility = Val(GetWord())
        .intelligence = Val(GetWord())
        .charisma = Val(GetWord())
        
        ' 读取武器熟练度
        .one_handed = Val(GetWord())
        .two_handed = Val(GetWord())
        .polearm = Val(GetWord())
        .archery = Val(GetWord())
        .crossbow = Val(GetWord())
        .throwing = Val(GetWord())
        .firearms = Val(GetWord())
        
        ' 读取技能
        For i = 0 To 41  ' 42个技能
            .skills(i) = Val(GetWord())
        Next i
        
        ' 读取面部特征
        .face_key_1 = GetWord()
        .face_key_2 = GetWord()
        .hair_color = Val(GetWord())
        .hair_texture = Val(GetWord())
        .face_texture = Val(GetWord())
        .voice = Val(GetWord())
        .body_color = Val(GetWord())
        .reserved_face = GetWord()
        
        ' 读取装备
        .item_count = Val(GetWord())
        If .item_count > 0 Then
            ReDim .items(1 To .item_count)
            For i = 1 To .item_count
                .items(i).item_id = GetWord()
                .items(i).imod = Val(GetWord())
            Next i
        End If
    End With
Next n
```

## 兵种升级系统

### 升级树结构

```vb
Type Type_Upgrade_Tree
    base_troop As Long         ' 基础兵种
    upgrade_1 As Long          ' 升级路径1
    upgrade_2 As Long          ' 升级路径2
    upgrade_cost As Long       ' 升级费用
    upgrade_exp As Long        ' 所需经验
End Type
```

### 升级条件

- 达到指定经验值
- 满足属性要求
- 拥有必要装备
- 派系限制检查

## 面部编码系统

### 面部特征码格式

面部特征码是一个64位整数，编码了角色的各种面部特征：

- 位 0-7: 面部模板
- 位 8-15: 皮肤颜色
- 位 16-23: 头发类型
- 位 24-31: 胡须类型
- 位 32-39: 头发颜色
- 位 40-47: 年龄
- 位 48-55: 面部变形
- 位 56-63: 保留

### 面部生成算法

```vb
Function GenerateRandomFace(gender As Long, age_min As Long, age_max As Long) As String
    Dim face_code As Integer64b
    
    ' 随机生成各个特征
    Call SetFaceBits(face_code, 0, 7, RandomRange(0, 15))      ' 面部模板
    Call SetFaceBits(face_code, 8, 15, RandomRange(0, 63))     ' 皮肤颜色
    Call SetFaceBits(face_code, 16, 23, RandomRange(0, 31))    ' 头发类型
    Call SetFaceBits(face_code, 24, 31, RandomRange(0, 15))    ' 胡须类型
    Call SetFaceBits(face_code, 32, 39, RandomRange(0, 63))    ' 头发颜色
    Call SetFaceBits(face_code, 40, 47, RandomRange(age_min, age_max))  ' 年龄
    
    GenerateRandomFace = Integer64bToHex(face_code)
End Function
```

## 数据验证规则

### 必填字段

- `dbName`: 必须唯一，不能为空
- `disname`: 显示名称，不能为空
- `faction`: 必须是有效的派系ID

### 数值范围

- 属性点: 3-63 (每个属性)
- 技能点: 0-15
- 武器熟练度: 0-500
- 等级: 1-63
- 经验值: >= 0

### 装备验证

- 装备ID必须在 `item_kinds1.txt` 中存在
- 装备类型必须与兵种匹配
- 英雄兵种可以装备任意物品

## 特殊兵种类型

### 玩家角色

- `is_hero = 1`
- 可以学习所有技能
- 装备无限制
- 可以升级属性

### NPC领主

- `is_hero = 1`
- 固定的面部特征
- 特定的装备配置
- 独特的对话和行为

### 普通士兵

- `is_hero = 0`
- 固定的技能配置
- 标准化装备
- 可以升级为高级兵种

## 常见问题

### Q: 如何创建新的兵种升级路径？

A: 在兵种的 `upgrade_flags` 字段中指定可升级的兵种ID，并设置相应的经验要求。

### Q: 面部特征码如何工作？

A: 面部特征码是一个编码字符串，包含了角色外观的所有信息，可以通过位操作进行解析和修改。

### Q: 为什么有些兵种无法装备某些物品？

A: 装备限制由物品的派系限制和兵种的派系属性决定，某些装备只能被特定派系使用。

### Q: 如何平衡兵种属性？

A: 需要考虑兵种的定位、成本、升级路径等因素，确保游戏平衡性。

---

*本文档基于魔球编辑器源码分析，详细描述了骑砍战团兵种数据的存储格式和解析方法。*
